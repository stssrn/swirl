import ReactMarkdown from "react-markdown";
import rehypeRaw from "rehype-raw";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";

import styles from "../styles/Markdown.module.scss";
import theme from "../lib/syntax-highlighting";

export default function Markdown({ markdown }: { markdown: string }) {
  return (
    <div className={styles.markdown}>
      <ReactMarkdown
        rehypePlugins={[rehypeRaw]}
        // https://github.com/remarkjs/react-markdown#use-custom-components-syntax-highlight
        components={{
          code({ node, inline, className, children, ...props }) {
            const match = /language-(\w+)/.exec(className || "");
            return !inline && match ? (
              <SyntaxHighlighter
                language={match[1]}
                PreTag="div"
                {...props}
                style={theme}
              >
                {String(children).replace(/\n$/, "")}
              </SyntaxHighlighter>
            ) : (
              <code className={className} {...props}>
                {children}
              </code>
            );
          },
        }}
      >
        {markdown}
      </ReactMarkdown>
    </div>
  );
}
