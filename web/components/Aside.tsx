import Image from "next/image";
import Link from "next/link";

import styles from "../styles/Aside.module.scss";

export default function Header({ children }: { children: JSX.Element }) {
  const height = 60;
  const aspectRatio = 3.2;

  return (
    <aside className={styles.aside}>
      <Link href="/">
        <a className={styles.logo} title="Homepage">
          <Image src="/logo.svg" alt="Swirl logo" height={height} width={height * aspectRatio} />
        </a>
      </Link>
      {children}
    </aside>
  );
}
