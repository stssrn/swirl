import styles from "../styles/Loading.module.scss";

export default function Loading({ item }: { item?: string }) {
  return (
    <p className={styles.loading}>
      {item ? `Loading ${item}...` : "Loading..."}
    </p>
  );
}
