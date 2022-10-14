export default function (...args: string[]): string {
  if (args.length > 0) {
    const trim = (s: string) => {
      const s1 = s.startsWith("/") ? s.substring(1) : s;
      return s1.endsWith("/") ? s1.substring(0, s1.length - 1) : s1;
    };
    return (
      `/${trim(args[0])}/` +
      args
        .slice(1)
        .map(trim)
        .filter((x) => x!.length > 0)
        .join("/")
    );
  } else {
    return "/";
  }
}
