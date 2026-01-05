import { readFileSync } from 'fs'
import { resolve } from 'path'
import { networkInterfaces } from 'os'
import { simple_encrypt_str } from '@noah-libjs/utils'
const cwd = process.cwd()
function safe_require(m_path: string) {
    try {
        return require(m_path)
    } catch { }
    try {
        let str = readFileSync(resolve(cwd, m_path)).toString()
        return JSON.parse(str)
    } catch { }
    return {}
}
export function client_macro_record(x: { [x: string]: any } = {}, raw = true) {
    const { WITH_THEME, APP_NAME, ENVIRONMENT_MODE, HOST_URL = '', PUBLIC_PATH = '/', API_PREFIX = '/', check_version, ...others } = process.env;
    const other_record = Object.entries(others).filter(_ => _[0].startsWith('LM_')).reduce(((sum, e) => ({ ...sum, [e[0]]: e[1] })), {})
    const devMode = ENVIRONMENT_MODE === 'development';
    const ips = Object.values(networkInterfaces()).flat().filter(ent => ent?.family === 'IPv4').map(ent => ent?.address)
    let m = {
        ...other_record,
        sb:'http://192.are.you.sb?',
        appName: APP_NAME,
        devMode,
        check_version,
        NODE_ENV: ENVIRONMENT_MODE,
        WITH_THEME,
        PUBLIC_PATH,
        API_PREFIX,
        PACKAGE_VERSION: safe_require('package.json').version,
        BUILDINFO: safe_require('build_meta.json'),
        __HOST_URL: simple_encrypt_str(HOST_URL)

    }
    const ret = {
        ...x,
        __DEV__: devMode,
        __LOCAL__: `(${JSON.stringify(ips)}.includes(location.hostname))`,
        APP_MACRO: raw ? JSON.stringify(m) : m,
    }
    return ret
}