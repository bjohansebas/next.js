import * as React from 'react'
import { unstable_after as after } from 'next/server'
import { setTimeout } from 'timers/promises'

export function generateStaticParams() {
  after(async () => {
    await setTimeout(500)
    throw new Error(
      `My cool error thrown inside unstable_after on route "/callback/[myParam]"`
    )
  })
  return [{ myParam: 'a' }, { myParam: 'b' }, { myParam: 'c' }]
}

export default async function Page(props) {
  const params = await props.params
  return <div>Param: {params.myParam}</div>
}
