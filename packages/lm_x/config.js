
const ReactRefreshPlugin = require("@rspack/plugin-react-refresh");
const this_webpack = require('@rspack/core');
const path = require('path');
const { client_macro_record } = require('@lm_fe/scripts')

const { defineConfig } = require('@rspack/cli');


const ENVIRONMENT_MODE = process.env.NODE_ENV
const id_dev = ENVIRONMENT_MODE === 'development';


const { HOST_URL = 'http://192.168.124.53:3351/', PUBLIC_PATH = '/', } = process.env;
const publicPath = PUBLIC_PATH;
const IGNORE_OLD = false;





module.exports = defineConfig(
  {
    mode: ENVIRONMENT_MODE,

    target: ['web', 'es5'],
    entry: {
      app: ['./src/index.tsx'],
      antd_base: ['antd', '@ant-design/icons'],

    },

    optimization: id_dev ? undefined : {
      sideEffects: true,
      runtimeChunk: 'single',
      splitChunks: {
        chunks: 'all'
      },
    },
    output: {
      clean: true,
      path: path.resolve('dist'),
      publicPath,
      filename: id_dev ? 'js/[name].js' : 'js/[name].[chunkhash:4].js',
      chunkFilename: id_dev ? 'js/[name].js' : 'js/[name].[chunkhash:4].js',

      environment: {
        arrowFunction: false,
        asyncFunction: false,
        const: false,
      },
    },
    // devtool: 'eval-cheap-module-source-map',
    devtool: false,

    resolve: {
      // mainFields: ['browser', 'main', 'module'],
      extensions: ['.ts', '.tsx', '.js', '.json'],
      modules: [path.resolve('node_modules')],
      alias: {
        '@': path.resolve('src'),
      },
    },
    externals: {
      // react: 'React',
      // 'react-dom': 'ReactDOM',
      // fabric: 'fabric'
    },

    module: {
      rules: [
        {
          test: /\.(ts|js)x?$/,
          exclude: /node_modules/,
          use: [
            { loader: 'thread-loader', options: { workers: 8 } },
            {
              loader: 'babel-loader', options: {
                cacheDirectory: true

              }
            },
          ],
        },

        (id_dev || IGNORE_OLD) ? false : {
          test: /\.(cjs|js)$/,
          include: /scroll-into-view-if-needed|compute-scroll-into-view|@ant-design/,

          use: {
            loader: 'babel-loader',
            options: {
              configFile: path.resolve('babel.config.dep.json')
            }
          }
        },

        (id_dev || IGNORE_OLD) ? false : {
          test: /\.(?:js)$/,
          include: /graphiql|dnd|graphql|meros|react|antd|rc-|@n1ru4l|punycode|dayjs|@lm_fe/,


          use: [
            { loader: 'thread-loader', options: { workers: 8 } },
            {
              loader: 'babel-loader',
              options: {
                configFile: path.resolve('babel.config.dep.json')
              }
            }
          ]
        },
        // {
        //   test: /\.(js)x?$/,
        //   include: /graphiql|dnd|graphql|meros|react|rc-util|@n1ru4l/,
        //   use: [
        //     { loader: 'thread-loader', options: { workers: 8 } },
        //     { loader: 'babel-loader', options: { cacheDirectory: true } },
        //   ],
        // },
        {
          test: /\.css$/,
          use: [
            { loader: 'style-loader' },
            { loader: 'css-loader', options: { sourceMap: false } },
            { loader: 'postcss-loader', options: { sourceMap: false } },
          ],
        },
        {
          test: /\.less$/,
          use: [
            { loader: 'style-loader' },
            { loader: 'css-loader', options: { sourceMap: false } },
            { loader: 'postcss-loader', options: { sourceMap: false } },
            {
              loader: 'less-loader',
              options: {
                sourceMap: false,
                lessOptions: {
                  // paths: [path.resolve(__dirname, './src'), path.resolve(__dirname, './node_modules/antd')],
                  javascriptEnabled: true,

                  // modifyVars: get_themeVariables(),

                  // modifyVars: getThemeVariables({
                  //   compact: true, // 开启紧凑模式
                  // }),
                },
              },
            },
          ],
        },
        {
          test: /\.(png|jpg|gif|svg|jpeg)$/,
          use: [
            {
              loader: 'file-loader',
              options: {
                // name: 'img/[name]_[hash:4].[ext]',
                name: id_dev ? '[path][name].[ext]' : 'img/[name]_[contenthash].[ext]',
                // esModule: false,
              },
            },
          ],
        },
        {
          test: /\.(woff2?|eot|ttf|otf)(\?.*)?$/,
          loader: 'url-loader',
          options: {
            limit: 20 * 1024,
            name: id_dev ? '[path][name].[ext]' : 'fonts/[name]-[contenthash].[ext]',
          },
        },
      ],
    },
    plugins: [
      ...[id_dev && new ReactRefreshPlugin()].filter(Boolean),

      id_dev ? false : new this_webpack.CssExtractRspackPlugin({
        filename: id_dev ? '[path][name].css' : 'css/[name].[chunkhash:4].css',
        // path: path.join(__dirname, '/dist'),
        ignoreOrder: true,
        // publicPath,
      }),

      id_dev ? false : new this_webpack.LightningCssMinimizerRspackPlugin(),
      id_dev ? false : new this_webpack.SwcJsMinimizerRspackPlugin({
        minimizerOptions: {
          // mangle: false, // mangle的作用是压缩变量名
          mangle: {
            reserved: ['ctx', 'React', 'ReactDOM'],
            // reserved: ['_'],
          },
          format: { // swc 未实现，所以不起作用
            braces: true,
            comments: false,
            beautify: true,
            semicolons: false,// 使用换行符而不是分号
          },
          compress: {
            drop_console: true,
          }
        }
      }),

      true ? false : new this_webpack.CopyRspackPlugin({
        patterns: [
          {
            from: path.resolve('public'),
            to: path.resolve('dist'),
          },


        ],
      }),
      // new WebpackBar(),
      new this_webpack.ProvidePlugin({
        'window.Quill': 'quill',
      }),
      new this_webpack.HtmlRspackPlugin({
        title: 'OBIS 产科信息管理系统',
        // template: './src/document_rs.html',
        template: 'index.ejs',
        favicon: './public/assets/favicon.png',
        chunks: ['app'],
        publicPath,
        templateParameters: {
          publicPath,
          title: 'OBIS 产科信息管理系统',

        },
        //   minify: {
        //     //删除注释
        //     removeComments: true,
        //     //删除空格
        //     collapseWhitespace: true,
        //     minifyCSS: true,
        //   },
        minify: true
      }),

      // this_webpack.HtmlWebpackTagsPlugin({
      //   scripts: [
      //     `lib/react/react.${devMode ? 'development' : 'production'}.js`,
      //     `lib/react/react-dom.${devMode ? 'development' : 'production'}.js`,
      //   ],
      //   publicPath,
      //   append: false,
      // }),
      // 该方法的两个参数都是正则，第一个参数表示要忽略的路径，第二个表示该资源所在目录，在该文件夹下引入的语言包都会被忽略
      // new webpack.IgnorePlugin(/\.\/locale/, /moment/),
      new this_webpack.DefinePlugin(client_macro_record()),
    ],


    devServer: {
      // contentBase: 'public',
      static: ['public'],
      // hotOnly: true,
      hot: true,
      host: '0.0.0.0',
      // host: 'local-ip',
      liveReload: true,
      // useLocalIp: true,
      client: {
        progress: true,
        overlay: false,
      },
      proxy: [

        {
          context: ['/api'],

          target: HOST_URL, // target host
          changeOrigin: true, // needed for virtual hosted sites
          ws: true, // proxy websockets
          pathRewrite: {

          },
          router: {

          },
        },

      ],
      historyApiFallback: {
        rewrites: [
          { from: /^\/(?!api).*$/, to: '/index.html' },

        ],
      },
    },

  }
)
