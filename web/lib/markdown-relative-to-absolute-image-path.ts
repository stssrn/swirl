export default function relativeToAbsoluteURL(
  markdown: string,
  base_url: string,
): string {
  // RegEx for capturing paths of embedded image links (needs improvement)

  const re = /(!\[[^\]]*\]\()([^http].*?)(?=\"|\))\w?(\))/g;

  // Safari doesn't support RegExp lookbehind/lookahead,
  // so the text surounding the path has to be captured as well
  return markdown.replaceAll(
    re,
    (_, before, path, after) => `${before}${base_url}${path.slice(1)}${after}`,
  );
}
