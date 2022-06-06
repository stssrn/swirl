import { Fragment } from "react";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";

import Loading from "./Loading";
import Markdown from "./Markdown";

import { PreviewData } from "../types/preview-data";

import styles from "../styles/FileViewer.module.scss";
import theme from "../lib/syntax-highlighting";
import relativeToAbsoluteURL from "../lib/markdown-relative-to-absolute-image-path";

export default function FileViewer({
  previewData,
  path,
  baseUrlPath,
  repo,
  host,
}: {
  previewData: PreviewData;
  path: string;
  baseUrlPath: string;
  repo: string;
  host: string;
}) {
  const src = `${baseUrlPath}${previewData.filePath}`;
  const filePath = path.split("/").slice(3).join("/");

  let previewElement: JSX.Element;
  switch (previewData.type) {
    case "audio":
      previewElement = (
        <audio controls>
          <source src={src} />
        </audio>
      );
      break;

    case "image":
      // eslint-disable-next-line @next/next/no-img-element
      previewElement = <img src={src} alt={previewData.filePath} />;
      break;

    case "video":
      previewElement = (
        <video controls>
          <source src={src} />
        </video>
      );
      break;

    case "text":
      previewElement = previewData.isMarkdown ? (
        <div className={styles.markdownWrapper}>
          <Markdown markdown={relativeToAbsoluteURL(previewData.content!, `${host}/repos/${repo}/raw/content`)} />
        </div>
      ) : (
        <SyntaxHighlighter
          language={previewData.filePath.split(".").slice(-1)[0]}
          style={theme}
        >
          {previewData.content!}
        </SyntaxHighlighter>
      );
      break;

    case "other":
      previewElement = <p>binary</p>;
      break;
  }

  return (
    <div className={styles.fileViewer}>
      <header>
        <h1 className={styles.fileName}>
          /
          {path
            .split("/")
            .slice(3)
            .map((part, i, arr) => (
              <Fragment key={i}>
                {part}
                {i !== arr.length - 1 && (
                  <>
                    /<wbr />
                  </>
                )}
              </Fragment>
            ))}
        </h1>
        <a href={src} className={styles.download}>
          download
        </a>
      </header>
      <div className={styles.filePreview}>
        {previewData.filePath === filePath ? (
          previewElement
        ) : (
          <Loading item="file" />
        )}
      </div>
    </div>
  );
}
