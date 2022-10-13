export default function (...args: string[]): string {
  return (
    "/" +
    args
      .map((x) => (x.endsWith("/") ? x.substring(1) : x))
      .filter((x) => x.length > 0)
      .reduce((acc, x) => `${acc}/${x}`)
  );
}
