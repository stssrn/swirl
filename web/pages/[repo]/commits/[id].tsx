import dynamic from "next/dynamic";
import { useRouter } from "next/router";

import Head from "next/head";
import Layout from "../../../components/Layout";

import getHostProps from "../../../lib/get-host-props";

const CommitDisplay = dynamic(
  () => import("../../../components/CommitDisplay"),
  { ssr: false },
);
const RepoLayout = dynamic(() => import("../../../components/RepoLayout"), {
  ssr: false,
});

interface Commit {
  author: Author;
  message: string;
  timestamp: number;
}

interface Author {
  email: string;
  name: string;
}

export async function getStaticPaths() {
  return { paths: [], fallback: true };
}

export async function getStaticProps() {
  return getHostProps();
}

export default function Commit({ host, soft_host }: { host: string, soft_host: string }) {
  const router = useRouter();

  const pathSegments = router.asPath.split("/");
  const repo = pathSegments[pathSegments.length - 3];

  let { id } = router.query;
  if (Array.isArray(id)) id = id[0];

  return (
    <Layout host={host} soft_host={soft_host} repo={repo}>
      <Head>
        <title>Commit {id?.slice(0, 7)} · {repo}</title>
      </Head>
      <RepoLayout host={host} repo={repo} currentTab="commits">
        <CommitDisplay host={host} repo={repo} id={id!} />
      </RepoLayout>
    </Layout>
  );
}
