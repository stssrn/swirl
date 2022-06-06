import cn from "classnames";
import Link from "next/link";
import { useRouter } from "next/router";
import styles from "../styles/FileList.module.scss";
import { TreeEntry, TreeResult } from "../types/tree";

// https://stackoverflow.com/a/20732091
function humanFileSize(size: number): string {
  const i = Math.floor( Math.log(size) / Math.log(1024) );
  const amount = Number((size / Math.pow(1024, i) ).toFixed(2));
  return  amount + ' ' + ['B', 'kB', 'MB', 'GB', 'TB'][i]
};

export default function FileList({ repo, tree }: { repo: string, tree: TreeResult }) {
  const { asPath } = useRouter();
  const dirPath = asPath.split("/").slice(3).join("/");
  const parentDir = asPath.split("/").slice(0, -1).join("/");

  return (
    <div className={styles.dirViewer}>
      <header className={styles.currentDir}>
        <h1>/{dirPath}</h1>
      </header>
      <ul className={styles.treeList}>
        { dirPath && (
          <li title="Go to parent directory">
            <Link href={parentDir}>
              <a>..</a>
            </Link>
          </li>
        )}
        {tree.node.entries!
          // Move directories to the top of the list
          .sort((a: TreeEntry, b: TreeEntry) => {
            if (a.entries && b.entries) return 0;
            if (a.entries && !b.entries) return -1;
            return 1;
          })
          .map((entry: TreeEntry) => (
            <li
              key={entry.id}
              className={cn({
                [styles.dir]: entry.entries,
              })}
              title={entry.name}
            >
              <Link href={`${asPath && asPath + "/"}${entry.name}`}>
                <a>{entry.name}</a>
              </Link>
              {entry.size &&
              <span className={styles.fileSize}>
                {humanFileSize(entry.size)}
              </span>
              }
            </li>
          ))}
      </ul>
    </div>
  );
}
