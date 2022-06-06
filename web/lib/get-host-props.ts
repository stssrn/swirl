export default function getHostProps() {
  const host: string = process.env.API_HOST!;
  const soft_host: string = process.env.SOFT_HOST!;

  return {
    props: {
      host,
      soft_host,
    },
  };
}
