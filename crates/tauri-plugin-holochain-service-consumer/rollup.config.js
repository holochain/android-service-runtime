import terser from '@rollup/plugin-terser'
import { nodeResolve } from '@rollup/plugin-node-resolve';

// NOTE rollup is only used to terse and bundle dependencies for holochain-env bundle
//
// It is not used for the tauri-commands bundle because we want ts declaration files,
// and there is a weird bug where the declaration files for both bundles are included in each bundle, in subdirectories.
// We are using tsc directly via package.json scripts instead.
// See https://github.com/rollup/plugins/issues/247
export default [
  {
    input: 'dist-js-tmp/holochain-env/index.js',
    output: [
      {
        file: "dist-js/holochain-env/index.min.js",
        format: 'esm',
      },
      {
        // Android resource filenames must be lowercase and have only a single '.' extension
        file: "android/src/main/res/raw/holochainenv.js",
        format: 'esm'
      },
    ],
    plugins: [
      terser(),      
      // Include imported dependencies in the output bundle
      nodeResolve(),
    ],
  },
]