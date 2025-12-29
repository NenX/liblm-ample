import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import babel, { getBabelOutputPlugin } from '@rollup/plugin-babel';
import typescript from '@rollup/plugin-typescript';
import { resolve, join } from 'path'

export default {
  input: {
    'index': resolve(__dirname, 'src/index.ts'),
  },

  plugins: [
    nodeResolve(),
    commonjs(),
    typescript({
      tsconfig: './tsconfig.json',
      // jsx: 'preserve',
    }),
    babel({
      presets: [['@babel/preset-env',

        // {
        //   targets: "node 14.0"
        // }
      ]],
      babelHelpers: 'bundled',
      extensions: ['.js', '.jsx', '.es6', '.es', '.mjs', '.ts', '.tsx']
    }),
    // copy({
    //   targets: [
    //     { src: 'src/asserts/**/*', dest: 'dist/asserts' }
    //   ]
    // })
  ],
  output: {
    dir: join(__dirname, 'dist'),
    format: 'esm',
    // banner: () => "#!/usr/bin/env node", //hash bang
    plugins: [
      getBabelOutputPlugin({
        presets: ['@babel/preset-env'],
      })
    ]
  },
  // external: ['chalk', 'yargs-parser', 'json2md'] // 增加了这一行。
}





