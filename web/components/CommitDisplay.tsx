import useSWR from "swr/immutable";

import ErrorComponent from "./Error";
import Loading from "./Loading";

import styles from "../styles/CommitDisplay.module.scss";

interface Commit {
  author: Author;
  message: string;
  timestamp: number;
}

interface Author {
  email: string;
  name: string;
}

export default function CommitDisplay({
  host,
  repo,
  id,
}: {
  host: string;
  repo: string;
  id: string;
}) {
  const { data: commit, error } = useSWR<Commit>(
    `${host}/repos/${repo}/commits/${id}`,
  );

  if (error) return <ErrorComponent message="Couldn't fetch commit" />

  return (
    <div className={styles.commitDisplay}>
      <h1>commit {id}</h1>
      {commit ? (
        <>
          <p>
            Author: {commit.author.name} &lt;{commit.author.email}&gt; <br />
            Date: {new Date(commit.timestamp * 1000).toString()}
          </p>
          <p className={styles.message}>{commit.message}</p>
        </>
      ) : (
        <Loading />
      )}
    </div>
  );
}
