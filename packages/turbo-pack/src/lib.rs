#![feature(trivial_bounds)]
#![feature(into_future)]

use asset::{Asset, AssetRef, AssetsSet, AssetsSetRef};
use module::ModuleRef;
use resolve::referenced_modules;
use turbo_tasks_fs::{FileContentRef, FileSystemPathRef};

pub mod asset;
mod ecmascript;
pub mod module;
pub mod reference;
pub mod resolve;
pub mod source_asset;
mod utils;

#[turbo_tasks::function]
pub async fn emit(module: ModuleRef, input_dir: FileSystemPathRef, output_dir: FileSystemPathRef) {
    let asset = nft_asset(module, input_dir, output_dir);
    emit_assets_recursive(asset);
}

#[turbo_tasks::function]
pub async fn emit_assets_recursive(asset: AssetRef) {
    let assets_set = asset.references().await;
    emit_asset(asset);
    for asset in assets_set.assets.iter() {
        emit_assets_recursive(asset.clone());
    }
}

#[turbo_tasks::function]
pub async fn nft_asset(
    module: ModuleRef,
    input_dir: FileSystemPathRef,
    output_dir: FileSystemPathRef,
) -> AssetRef {
    let new_path = FileSystemPathRef::rebase(
        module.get().await.path.clone(),
        input_dir.clone(),
        output_dir.clone(),
    );

    NftAssetSource {
        path: new_path,
        module,
        input_dir,
        output_dir,
    }
    .into()
}

#[turbo_tasks::value(Asset)]
#[derive(Hash, PartialEq, Eq)]
struct NftAssetSource {
    path: FileSystemPathRef,
    module: ModuleRef,
    input_dir: FileSystemPathRef,
    output_dir: FileSystemPathRef,
}

#[turbo_tasks::value_impl]
impl Asset for NftAssetSource {
    async fn path(&self) -> FileSystemPathRef {
        self.path.clone()
    }

    async fn content(&self) -> FileContentRef {
        self.module.get().await.path.clone().read()
    }

    async fn references(&self) -> AssetsSetRef {
        let modules = referenced_modules(self.module.clone());
        let mut assets = Vec::new();
        for module in modules.await.modules.iter() {
            assets.push(nft_asset(
                module.clone(),
                self.input_dir.clone(),
                self.output_dir.clone(),
            ));
        }
        AssetsSet { assets }.into()
    }
}

#[turbo_tasks::function]
pub fn emit_asset(asset: AssetRef) {
    asset.path().write(asset.content());
}
