import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import babel, { getBabelOutputPlugin } from '@rollup/plugin-babel';
import typescript from '@rollup/plugin-typescript';
import copy from 'rollup-plugin-copy'
import { resolve, join } from 'path'
import { readdirSync } from 'fs';
const multiModules = readdirSync(resolve(__dirname, './src/cli'))
  .reduce((a, b) => ({ ...a, [b]: resolve(__dirname, './src/cli', b, 'index.ts') }), {})

export default {
  input: {
    'index': resolve(__dirname, 'src/index.ts'),
    ...multiModules,
  },

  plugins: [
    nodeResolve(),
    commonjs(),
    typescript({
      tsconfig: './tsconfig.json',
      jsx: 'preserve',
    }),
    babel({
      presets: [['@babel/preset-env',

        {
          targets: "node 14.0"
        }
      ]],
      babelHelpers: 'bundled',
      extensions: ['.js', '.jsx', '.es6', '.es', '.mjs', '.ts', '.tsx']
    }),
    copy({
      targets: [
        { src: 'src/asserts/**/*', dest: 'dist/asserts' }
      ]
    })
  ],
  output: {
    dir: join(__dirname, 'dist'),
    format: 'cjs',
    banner: () => "#!/usr/bin/env node", //hash bang
    plugins: [
      getBabelOutputPlugin({
        presets: ['@babel/preset-env'],
      })
    ]
  },
  external: [
    // ...Object.keys(require('./package.json').dependencies || {}),
    ...Object.keys(require('./package.json').peerDependencies || {}),
    ...Object.keys(require('./package.json').dependencies || {}),
  ]
}





