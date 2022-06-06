import dynamic from "next/dynamic";
import useSWR from "swr/immutable";
import { useRouter } from "next/router";
import { useState } from "react";

import Head from "next/head";
import Link from "next/link";

import ErrorComponent from "../../../components/Error";
import Layout from "../../../components/Layout";
import Loading from "../../../components/Loading";

import getHostProps from "../../../lib/get-host-props";

import styles from "../../../styles/Commits.module.scss";

const RepoLayout = dynamic(() => import("../../../components/RepoLayout"), {
  ssr: false,
});

interface Commit {
  id: string;
  message: string;
}

export async function getStaticPaths() {
  return { paths: [], fallback: true };
}

export async function getStaticProps() {
  return getHostProps();
}

export default function Commits({
  host,
  soft_host,
}: {
  host: string;
  soft_host: string;
}) {
  const router = useRouter();
  let { repo } = router.query;
  if (Array.isArray(repo)) repo = repo[0];

  const [branch, setBranch] = useState("");
  const pageSize = 30;
  const [page, setPage] = useState(0);

  const { data: commits, error } = useSWR<Commit[]>(
    () =>
      branch &&
      `${host}/repos/${repo}/commits?branch=${branch}&page=${page}&limit=${pageSize}`,
  );
  //const { data: commits} = useSWR<Commit[]>(() => host + "/repos/" + repo + "/commits?branch=" + branch + "&page=" + page + "&limit=" + pageSize)

  if (error)
    return (
      <Layout host={host} soft_host={soft_host} repo={repo!}>
        <RepoLayout
          currentTab="commits"
          host={host}
          repo={repo!}
          branch={branch}
          branchSetter={setBranch}
        >
          <ErrorComponent message="Couldn't fetch commits" />
        </RepoLayout>
      </Layout>
    );

  return (
    <Layout host={host} soft_host={soft_host} repo={repo!}>
      <Head>
        <title>Commits · {repo}</title>
      </Head>
      <RepoLayout
        currentTab="commits"
        host={host}
        repo={repo!}
        branch={branch}
        branchSetter={setBranch}
      >
        <div className={styles.container}>
          {commits ? (
            <ul className={styles.commits}>
              {commits.map((commit: Commit) => (
                <Link key={commit.id} href={`${router.asPath}/${commit.id}`}>
                  <a>
                    <li>
                      <span className={styles.id}>{commit.id.slice(0, 7)}</span>{" "}
                      <span className={styles.message}>{commit.message}</span>
                    </li>
                  </a>
                </Link>
              ))}
            </ul>
          ) : (
            <Loading item="commits" />
          )}
          {commits && (
            <div className={styles.pageNavigation}>
              {page > 0 && (
                <a
                  onClick={() => setPage((prevPage) => prevPage - 1)}
                  className={styles.back}
                >
                  back
                </a>
              )}
              {commits.length === pageSize && (
                <a
                  onClick={() => setPage((prevPage) => prevPage + 1)}
                  className={styles.next}
                >
                  next
                </a>
              )}
            </div>
          )}
        </div>
      </RepoLayout>
    </Layout>
  );
}
