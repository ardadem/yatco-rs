import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';

export function highlightAuto(code: string) {
  const result = hljs.highlightAuto(code);
  return {
    value: result.value,
    language: result.language || ''
  };
}
