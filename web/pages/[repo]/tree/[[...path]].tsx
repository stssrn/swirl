import dynamic from "next/dynamic";
import { useEffect, useState } from "react";
import { useRouter } from "next/router";

import Head from "next/head";

import Layout from "../../../components/Layout";
import Loading from "../../../components/Loading";

import getFileType from "../../../lib/get-file-type";
import getHostProps from "../../../lib/get-host-props";

import { PreviewData } from "../../../types/preview-data";
import { TreeEntry, TreeResult } from "../../../types/tree";

import styles from "../../../styles/Tree.module.scss";

const FileList = dynamic(() => import("../../../components/FileList"), {
  ssr: false,
});
const FileViewer = dynamic(() => import("../../../components/FileViewer"), {
  ssr: false,
});
const RepoLayout = dynamic(() => import("../../../components/RepoLayout"), {
  ssr: false,
});

export async function getStaticPaths() {
  return { paths: [], fallback: true };
}

export async function getStaticProps() {
  return getHostProps();
}

function entryFinder(tree: TreeEntry, path: string[]) {
  if (path) {
    let treeResult: TreeResult = { node: tree, isFile: false };
    for (const i in path) {
      if (treeResult.isFile) break;
      const result = treeResult.node.entries?.filter(
        (entry) => entry.name === path[i],
      )[0];
      if (result?.entries) {
        // Continue searching if found file is a directory
        treeResult = { node: result, isFile: false };
      } else if (result) {
        // There are no entries, but there is a result, a file has been found
        treeResult = { node: result, isFile: true };
        break;
      } else {
        throw `Couldn't find target ${path}`;
      }
    }
    return treeResult;
  }
}

export default function Tree({
  host,
  soft_host,
}: {
  host: string;
  soft_host: string;
}) {
  const router = useRouter();
  let { repo, path } = router.query;
  if (Array.isArray(repo)) repo = repo[0];
  if (typeof path === "string") path = [path];

  const [branch, setBranch] = useState("");
  let [tree, setTree] = useState<any>();
  const [previewData, setPreviewData] = useState<PreviewData | null>();
  const [treeBranch, setTreeBranch] = useState("");

  const { asPath } = useRouter();
  const pathSegments = asPath.split("/");

  useEffect(() => {
    if (branch) {
      if (!tree || branch !== treeBranch) {
        const getTree = async (branch: string) => {
          const data = await fetch(
            `${host}/repos/${repo}/branches/${branch}/tree`,
          );
          const json = await data.json();

          setTree(json);
          setTreeBranch(branch);
        };
        getTree(branch);
      }
    }
  }, [branch, host, repo, tree, treeBranch]);

  const baseUrlPath = `${host}/repos/${repo}/raw/content/`;
  const filePath = pathSegments.slice(3).join("/");

  useEffect(() => {
    if (tree && tree.isFile && filePath !== previewData?.filePath) {
      const fileType = getFileType(tree.node.name);
      if (fileType == "other") {
        const otherHandler = async (filePath: string) => {
          const data = await fetch(
            `${host}/repos/${repo}/raw/is_bin/${filePath}?branch=${branch}`,
          );
          const isBin = await data.json();

          if (isBin) {
            setPreviewData({
              type: "other",
              filePath,
            });
          } else {
            const data = await fetch(`${baseUrlPath}${filePath}?branch=${branch}`);
            const text = await data.text();
            const isMarkdown = filePath.split(".").slice(-1)[0] === "md";
            setPreviewData({
              type: "text",
              isMarkdown,
              content: text,
              filePath,
            });
          }
        };
        otherHandler(filePath);
      } else {
        setPreviewData({
          type: fileType,
          filePath,
        });
      }
    }
  }, [baseUrlPath, branch, filePath, host, pathSegments, previewData, repo, tree]);

  if (tree) {
    // Find target node
    try {
      tree = path ? entryFinder(tree, path) : { node: tree, isFile: false };
    } catch {
      return (
        <Layout host={host} soft_host={soft_host} repo={repo!}>
          <Head>
            <title>
              /{path && path.join("/")} at {branch} · {repo}
            </title>
          </Head>
          <RepoLayout
            currentTab="tree"
            host={host}
            repo={repo!}
            branch={branch}
            branchSetter={setBranch}
          >
            <div className={styles.errorContainer}>
              <p className="error">path {path?.join("/")} was not found</p>
            </div>
          </RepoLayout>
        </Layout>
      );
    }
  }

  // File Viewer
  if (tree && tree.isFile && previewData) {
    return (
      <Layout host={host} soft_host={soft_host} repo={repo!}>
        <Head>
          <title>
            /{path && path.join("/")} at {branch} · {repo}
          </title>
        </Head>
        <RepoLayout
          currentTab="tree"
          host={host}
          repo={repo!}
          branch={branch}
          branchSetter={setBranch}
        >
          <FileViewer
            previewData={previewData}
            path={asPath}
            baseUrlPath={baseUrlPath}
            repo={repo!}
            host={host}
          />
        </RepoLayout>
      </Layout>
    );
  }

  // Directory View
  if (tree && !tree.isFile)
    return (
      <Layout host={host} soft_host={soft_host} repo={repo!}>
        <Head>
          <title>
            /{path && path.join("/")} at {branch} · {repo}
          </title>
        </Head>
        <RepoLayout
          currentTab="tree"
          host={host}
          repo={repo!}
          branch={branch}
          branchSetter={setBranch}
        >
          <FileList repo={repo!} tree={tree} />
        </RepoLayout>
      </Layout>
    );

  return (
    <Layout host={host} soft_host={soft_host} repo={repo!}>
      <Head>
        <title>
          /{path && path.join("/")} at {branch} · {repo}
        </title>
      </Head>
      <RepoLayout
        currentTab="tree"
        host={host}
        repo={repo!}
        branch={branch}
        branchSetter={setBranch}
      >
        <div className={styles.dirViewer}>
          <Loading item="tree" />
        </div>
      </RepoLayout>
    </Layout>
  );
}
