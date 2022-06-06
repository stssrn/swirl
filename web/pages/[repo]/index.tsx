import dynamic from "next/dynamic";
import useSWR from "swr/immutable";
import { useRouter } from "next/router";
import { useState } from "react";

import Head from "next/head";

import ErrorComponent from "../../components/Error";
import Layout from "../../components/Layout";
import Loading from "../../components/Loading";
import Markdown from "../../components/Markdown";
import styles from "../../styles/Repo.module.scss";

import getHostProps from "../../lib/get-host-props";
import relativeToAbsoluteURL from "../../lib/markdown-relative-to-absolute-image-path";

const RepoLayout = dynamic(() => import("../../components/RepoLayout"), {
  ssr: false,
});

interface Repo {
  name: string;
  note: string;
  readme: string;
  repo: string;
}

export async function getStaticPaths() {
  return { paths: [], fallback: true };
}

export async function getStaticProps() {
  return getHostProps();
}

const fetcher = (url: string) => fetch(url).then((r) => r.text());

export default function Repo({
  host,
  soft_host,
}: {
  host: string;
  soft_host: string;
}) {
  const { data: repos, error } = useSWR(`${host}/repos`);
  const [branch, setBranch] = useState("");

  const router = useRouter();
  let { repo } = router.query;
  if (Array.isArray(repo)) repo = repo[0];

  const repoInfo = repos?.find(
    (fetchedRepo: Repo) => fetchedRepo.repo === repo,
  );
  const { data: readme } = useSWR<string>(
    () =>
      repoInfo?.readme &&
      host + "/repos/" + repo + "/raw/content/" + repoInfo.readme,
    fetcher,
  );

  if (error)
    return (
      <Layout host={host} soft_host={soft_host} repo={repo!}>
        <RepoLayout
          currentTab="readme"
          host={host}
          repo={repo!}
          branch={branch}
          branchSetter={setBranch}
        >
          <ErrorComponent message="couldn't fetch readme" />
        </RepoLayout>
      </Layout>
    );

  if (!repos)
    return (
      <Layout host={host} soft_host={soft_host} repo={repo!}>
        <RepoLayout
          currentTab="readme"
          host={host}
          repo={repo!}
          branch={branch}
          branchSetter={setBranch}
        >
          <div className={styles.loadContainer}>
            <Loading item="readme" />
          </div>
        </RepoLayout>
      </Layout>
    );

  return (
    <Layout host={host} soft_host={soft_host} repo={repo!}>
      <Head>
        <title>ReadMe · {repo}</title>
      </Head>
      <RepoLayout
        currentTab="readme"
        host={host}
        repo={repo!}
        branch={branch}
        branchSetter={setBranch}
      >
        <header className={styles.cloneBanner}>
          <h1>{repo}</h1>
          <p>
            git clone {soft_host}/{repo}
          </p>
        </header>
        <div className={styles.main}>
          {readme ? (
            <Markdown markdown={relativeToAbsoluteURL(readme, `${host}/repos/${repo}/raw/content`)} />
          ) : repoInfo?.readme ? (
            <Loading item="readme" />
          ) : (
            <p>{repoInfo?.note}</p>
          )}
        </div>
      </RepoLayout>
    </Layout>
  );
}
