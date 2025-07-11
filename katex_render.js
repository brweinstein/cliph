import { renderMathInElement as katexRender } from "https://cdn.jsdelivr.net/npm/katex@0.16.4/dist/contrib/auto-render.min.js";

export function renderMathInElement(element) {
    katexRender(element, {
        delimiters: [
            {left: "$", right: "$", display: false},
            {left: "\\(", right: "\\)", display: false},
            {left: "\\[", right: "\\]", display: true},
        ],
        throwOnError: false,
    });
}
