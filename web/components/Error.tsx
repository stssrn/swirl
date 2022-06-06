import styles from '../styles/Error.module.scss';

export default function Error ({message}: {message: string}) {
  return <p className={styles.error}>{message}</p>
}
