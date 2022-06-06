import classNames from "classnames";
import useSWR from "swr/immutable";

import Link from "next/link";
import Loading from "./Loading";

import Repo from "../types/repos";

import styles from "../styles/RepoList.module.scss";

export default function RepoList({
  host,
  currentRepo,
}: {
  host: string;
  currentRepo: string;
}) {
  const { data, error } = useSWR(`${host}/repos`);

  if (error) return <></>;

  if (!data)
    return (
      <div className={styles.status}>
        <Loading />
      </div>
    );

  return (
    <ul className={styles.repos}>
      {data &&
        data
          // Sort into lexicographic order
          .sort((a: Repo, b: Repo) => a.name.localeCompare(b.name))
          .map((repo: Repo) => (
            <li
              key={repo.repo}
              title={repo.note}
              className={classNames({
                ["selected"]: repo.repo === currentRepo,
              })}
            >
              <Link href={`/${repo.repo}`}>
                <a>{repo.name}</a>
              </Link>
            </li>
          ))}
    </ul>
  );
}
