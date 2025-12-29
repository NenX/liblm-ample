
import fs from 'fs';
import { colorLog, successLog, warnLog } from 'src/utils';


const who = 'copy-static'
export function warnLogImpl(txt: string) {
    warnLog(txt, who)
}
export function successLogImpl(txt: string) {
    successLog(txt, who)
}


export function findEntries(base_path: string, create = false) {
    const exists = fs.existsSync(base_path)
    if (!exists) {
        if (create) {
            fs.mkdirSync(base_path)
            return []
        }
        colorLog(`${base_path} 不存在！`, 'bgRed')
        return
    }
    const stat = fs.statSync(base_path)
    if (!stat.isDirectory()) {
        colorLog(`${base_path} 不是文件夹！`, 'bgRed')
        return
    }
    const assertNames = fs.readdirSync(base_path)

    return assertNames
}


