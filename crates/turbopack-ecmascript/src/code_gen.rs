use swc_ecma_visit::{AstParentKind, VisitMut};
use turbopack_core::chunk::ChunkingContextVc;

use crate::chunk::EcmascriptChunkContextVc;

/// impl of code generation inferred from a AssetReference.
/// This is rust only and can't be implemented by non-rust plugins.
#[turbo_tasks::value(shared, serialization: none, eq: manual, into: new, cell: new)]
pub struct CodeGeneration {
    /// ast nodes matching the span will be visitor by the visitor
    #[trace_ignore]
    pub visitors: Vec<(Vec<AstParentKind>, Box<dyn VisitorFactory>)>,
}

pub trait VisitorFactory: Send + Sync {
    fn create<'a>(&'a self) -> Box<dyn VisitMut + Send + Sync + 'a>;
}

#[turbo_tasks::value_trait]
pub trait CodeGenerationReference {
    fn code_generation(
        &self,
        chunk_context: EcmascriptChunkContextVc,
        context: ChunkingContextVc,
    ) -> CodeGenerationVc;
}
