import type { AppProps } from 'next/app'
import { SWRConfig } from 'swr'

import '../styles/globals.scss'

export default function Swirl({ Component, pageProps }: AppProps) {
  return (
    <SWRConfig
      value={{
        fetcher: (url) => fetch(url).then((res) => res.json()),
      }}
      >
        <Component {...pageProps} />
      </SWRConfig>
  )
}
