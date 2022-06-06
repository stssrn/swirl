import cn from "classnames";
import { useRouter } from "next/router";
import React, {
  Dispatch,
  SetStateAction,
  useEffect,
  useLayoutEffect,
} from "react";
import useSWR from "swr/immutable";

import Link from "next/link";

import styles from "../styles/RepoLayout.module.scss";

const tabs = ["readme", "tree", "commits"] as const;
type Tabs = typeof tabs[number];

interface Branch {
  name: string;
}

export default function RepoLayout({
  host,
  repo,
  currentTab,
  home = false,
  branch,
  branchSetter,
  children,
}: {
  host: string;
  repo: string;
  currentTab: Tabs;
  home?: boolean;
  branch?: string;
  branchSetter?: Dispatch<SetStateAction<string>>;
  children: JSX.Element | JSX.Element[];
}) {
  const { data, error } = useSWR(`${host}/repos/${repo}/branches`);

  const router = useRouter();
  const path = router.asPath.split("/");
  const lastPathPart = path[path.length - 1];

  const handleChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    if (branchSetter) branchSetter(event.target.value);
  };

  useEffect(() => {
    if (branch && data?.includes(branch)) sessionStorage.setItem(`branch-${repo}`, branch);
  }, [branch, data, repo]);

  useLayoutEffect(() => {
    if (branchSetter){
      if (!branch || path[1] !== repo || !data?.includes(branch)) {
        if (sessionStorage.getItem(`branch-${repo}`)) {
          branchSetter(sessionStorage.getItem(`branch-${repo}`)!)
        } else if (data?.includes("main")) {
          branchSetter("main")
        } else if (data?.includes("master")) {
          branchSetter("master")
        } else if (data) branchSetter(data[0]);
      } else if (branch) { branchSetter(branch) };
    };
  }, [branch, branchSetter, data, lastPathPart, path, repo]);

  return (
    <>
      <header>
        <nav className={styles.nav}>
          <ul>
            {!home &&
              tabs.map((tab) => {
                const parent = router.asPath.split("/").slice(0, 2).join("/");
                const target =
                  tab === lastPathPart ||
                  (tab === "readme" && lastPathPart === "[repo]")
                    ? router.asPath
                    : lastPathPart === "[repo]"
                      ? `${router.asPath}/${tab}`
                      : tab === "readme"
                        ? parent
                        : `${parent}/${tab}`;

                return (
                  <li
                    key={tab}
                    className={cn({
                      ["selected"]: tab === currentTab,
                    })}
                  >
                    <Link href={target}>
                      <a>{tab}</a>
                    </Link>
                  </li>
                );
              })}
          </ul>
          {data && branch && (
            <select
              id="branch"
              className={styles.select}
              value={branch}
              onChange={handleChange}
            >
              {data
                // Filter out tags
                .filter((branch: string) => !branch.includes("/"))
                .map((branch: string) => (
                <option key={branch} value={branch}>
                  {branch}
                </option>
              ))}
            </select>
          )}
        </nav>
      </header>
      <main className={styles.main}>{children}</main>
    </>
  );
}
