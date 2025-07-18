// katex_render.js
// Define a global helper to render math in an element using KaTeX auto-render

function renderMathInElementHelper(elem) {
  renderMathInElement(elem, {
    delimiters: [
      { left: "$", right: "$", display: false },
      { left: "\\(", right: "\\)", display: false },
      { left: "\\[", right: "\\]", display: true },
    ],
    throwOnError: false,
  });
}

// expose globally for wasm-bindgen extern
window.renderMathInElementHelper = renderMathInElementHelper;
