import Head from "next/head";
import useSWR from "swr/immutable";

import Layout from "../components/Layout";
import Loading from "../components/Loading";
import Markdown from "../components/Markdown";

import getHostProps from "../lib/get-host-props";
import relativeToAbsoluteURL from "../lib/markdown-relative-to-absolute-image-path";

import styles from "../styles/Home.module.scss";

export async function getStaticProps() {
  return getHostProps();
}

const fetcher = (url: string) => fetch(url).then((r) => r.text());

export default function Home({
  host,
  soft_host,
}: {
  host: string;
  soft_host: string;
}) {
  const { data: readme, error } = useSWR<string>(
    () => host + "/repos/config/raw/readme",
    fetcher,
  );

  return (
    <Layout host={host} soft_host={soft_host} repo="config">
      <Head>
        <title>Home · Swirl</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={styles.main}>
        {readme ? (
          <Markdown markdown={relativeToAbsoluteURL(readme, soft_host)} />
        ) : error ? (
          <p className="error">Swirl server error</p>
        ) : (
          <Loading item="home" />
        )}
      </main>
    </Layout>
  );
}
