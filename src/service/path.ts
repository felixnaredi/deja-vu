export default function (...args: string[]): string {
  if (args.length > 1) {
    const trim = (s: string) => (s.endsWith("/") ? s.substring(1) : s);
    return (
      `/${trim(args[0])}` +
      args
        .slice(1)
        .map(trim)
        .filter((x) => x!.length > 0)
        .reduce((acc, x) => `${acc}/${x}`)
    );
  } else {
    return "/";
  }
}
