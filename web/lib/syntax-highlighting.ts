import { CSSProperties } from "react";

const dark: {[key: string]: CSSProperties} = {
    "code[class*=\"language-\"]": {
        "color": "#FFF",
        "direction": "ltr",
        "textAlign": "left",
        "whiteSpace": "pre",
        "wordSpacing": "normal",
        "wordBreak": "normal",
        "MozTabSize": "4",
        "OTabSize": "4",
        "tabSize": "4",
        "WebkitHyphens": "none",
        "MozHyphens": "none",
        "msHyphens": "none",
        "hyphens": "none"
    },
    "pre[class*=\"language-\"]": {
        "color": "#c5c8c6",
        "direction": "ltr",
        "textAlign": "left",
        "whiteSpace": "pre",
        "wordSpacing": "normal",
        "wordBreak": "normal",
        "MozTabSize": "4",
        "OTabSize": "4",
        "tabSize": "4",
        "WebkitHyphens": "none",
        "MozHyphens": "none",
        "msHyphens": "none",
        "hyphens": "none",
        "padding": "0ch 1.67ch",
        "overflow": "auto",
        "background": "#1E1E1E"
    },
    ":not(pre) > code[class*=\"language-\"]": {
        "background": "#1d1f21",
        "padding": ".1em",
    },
    "comment": {
        "color": "#949494"
    },
    "prolog": {
        "color": "#7C7C7C"
    },
    "doctype": {
        "color": "#F5A279"
    },
    "cdata": {
        "color": "#7C7C7C"
    },
    "punctuation": {
        "color": "#E3E3BB"
    },
    ".namespace": {
        "color": "#FF5F87"
    },
    "property": {
        "color": "#46C9FF"
    },
    "keyword": {
        "color": "#00AAFF"
    },
    "tag": {
        "color": "#CEA4F6"
    },
    "class-name": {
        "color": "#F1F1F1",
        "textDecoration": "underline",
        "fontWeight": "bold"
    },
    "boolean": {
        "color": "#46C9FF"
    },
    "constant": {
        "color": "#99CC99"
    },
    "symbol": {
        "color": "#f92672"
    },
    "deleted": {
        "color": "#f92672"
    },
    "number": {
        "color": "#69FFB7"
    },
    "selector": {
        "color": "#CEA4F6",
    },
    "attr-name": {
        "color": "#A8A6F9"
    },
    "string": {
        "color": "#F5A279"
    },
    "char": {
        "color": "#C69669"
    },
    "builtin": {
        "color": "#8D86FF"
    },
    "inserted": {
        "color": "#00D787"
    },
    "variable": {
        "color": "#C6C5FE"
    },
    "operator": {
        "color": "#EF8080"
    },
    "entity": {
        "color": "#FFFFB6",
        "cursor": "help"
    },
    "url": {
        "color": "#96CBFE"
    },
    ".language-css .token.string": {
        "color": "#87C38A"
    },
    ".style .token.string": {
        "color": "#87C38A"
    },
    "atrule": {
        "color": "#F9EE98"
    },
    "attr-value": {
        "color": "#F5A279"
    },
    "function": {
        "color": "#00D787"
    },
    "regex": {
        "color": "#E9C062"
    },
    "important": {
        "color": "#fd971f",
        "fontWeight": "bold"
    },
    "bold": {
        "fontWeight": "bold"
    },
    "italic": {
        "fontStyle": "italic"
    }
}

export default dark;
