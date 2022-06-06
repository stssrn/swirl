import Aside from "./Aside";
import RepoList from "./RepoList";

import styles from "../styles/Layout.module.scss";

export default function Layout({
  host,
  soft_host,
  children,
  repo,
}: {
  host: string;
  soft_host: string;
  children: JSX.Element | JSX.Element[];
  repo: string;
}) {
  return (
    <>
      <div className={styles.topBanner}>
        view in the TUI {soft_host} -{" "}
        <a
          href="https://github.com/charmbracelet/soft-serve/blob/main/README.md"
          target="_blank"
          rel="noreferrer"
        >
          more info
        </a>
      </div>
      <div className={styles.container}>
        <Aside>
          <RepoList host={host} currentRepo={repo} />
        </Aside>
        <div className={styles.mainColumn}>{children}</div>
      </div>
    </>
  );
}
