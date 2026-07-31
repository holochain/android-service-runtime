import terser from '@rollup/plugin-terser';
import { nodeResolve } from '@rollup/plugin-node-resolve';

// Bundle + minify the holochain-env injection script, inlining its deps
// (@msgpack/msgpack) so it can be injected into a webview as a single script.
export default [
  {
    input: 'dist-js-tmp/holochain-env/index.js',
    output: [
      {
        file: 'dist-js/holochain-env/index.min.js',
        format: 'esm',
      },
    ],
    plugins: [terser(), nodeResolve()],
  },
];
