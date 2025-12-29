
import { appendFileSync, existsSync, mkdirSync, copyFileSync, rmSync, readFileSync } from 'fs';
import { resolve } from 'path';
import { successLog, warnLog } from 'src/utils';
import { getArgs, successLogImpl, warnLogImpl } from './utils';

const args = getArgs()



const DIR_NAME = 'form_config'
const ENV_LIST = ['华医', '南医增城', '广三', '越秀妇幼', '建瓯']
const FOMR_CONFIG_TEMPLATE = () => `
import { defineFormConfig } from "@lm_fe/service";

export default defineFormConfig([])
`
// const LOAD_FORM_TEMPLATE = (env_list: string[]) => `
// import { mchcEnv } from '@lm_fe/env'

// export function load_form_config() {
// ${env_list.map(e => `    if (mchcEnv.is('${e}')) return import('./${e}')`).join('\r\n')}
//     return import('./default')
// }
// `

const LOAD_FORM_TEMPLATE1 = (env_list: string[]) => `
${env_list.map(e => `    if (mchcEnv.is('${e}')) return import('./${e}')`).join('\r\n')}
    return import('./default')
`
const LOAD_FORM_TEMPLATE = (env_list: string[]) => `
export function load_form_config() {
${env_list.map(e => `    if (mchcEnv.is('${e}')) return import('./${e}')`).join('\r\n')}
    return import('./default')
}
`
const LOAD_FORM_IMPORT = (env_list: string[]) => `
${env_list.map(e => `import { default as ${e}_config } from './${e}'`).join('\r\n')}
import { default as default_config } from './default'
`


function main() {
    if (!args) return
    let { rm, target, names, nolazy } = args
    const target_dir_name = target || DIR_NAME
    const target_dir_path = resolve(process.cwd(), target_dir_name)
    const is_exists = existsSync(target_dir_path,)

    const _env_list = names ? (names?.split?.(',') ?? ENV_LIST) : ENV_LIST
    const env_list = _env_list.filter(_ => _).map(_ => _.trim())
    if (rm) {
        if (!is_exists) return warnLogImpl(`${target_dir_path} 不存在!!`)

        rmSync(target_dir_path, { recursive: true })
        return successLogImpl(`${target_dir_path} 删除成功!! ✅`,)
    }
    if (is_exists) return warnLogImpl(`${target_dir_path} 已存在!!`)


    mkdirSync(target_dir_path)
    env_list.forEach(_ => {
        const r_path = resolve(target_dir_path, `${_}.tsx`)
        appendFileSync(r_path, FOMR_CONFIG_TEMPLATE())
    })
    appendFileSync(resolve(target_dir_path, `default.tsx`), FOMR_CONFIG_TEMPLATE())
    // copyFileSync(resolve(__dirname, `asserts/form_config_tpl.tsx`), resolve(target_dir_path, `default.tsx`))
    const tplStr = readFileSync(resolve(__dirname, `asserts/form_config_tpl.tsx`))
        .toString()
        .replace('/*[lazy]*/', nolazy ? '' : LOAD_FORM_TEMPLATE(env_list))
        .replace('/*[import1]*/', nolazy ? LOAD_FORM_IMPORT(env_list) : '')
        .replace('/*[import2]*/', nolazy ? '    const [load_form_config, set_load_form_config] = useState(default_config)' : '')

    // appendFileSync(resolve(target_dir_path, `index.tsx`), LOAD_FORM_TEMPLATE(env_list))
    appendFileSync(resolve(target_dir_path, `Form.tsx`), tplStr)

    successLogImpl(`${target_dir_path} 生成成功!! ✅`,)
}






main()
