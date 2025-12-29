import { readdirSync, statSync } from 'fs';
import { resolve } from 'path';

import { successLog, warnLog } from 'src/utils';
import argsParse from 'yargs-parser';

const MAX_LEVEL = 999
const EXCLUDE_PATH = ['components', 'copy']
const PREFIX = ''
const ENTRYPATH = ['index.tsx', 'index.nl.tsx']
const who = 'gen-routes'

const argv = argsParse(process.argv.slice(2))


export const ANNOTATION = '// 这个文件是生成的，请不要手动修改'
export const USAGE = `usage: gen-routes --source=/src/pages --target=/src/routes/meta.ts [--withmd] [--underline] [--help]`

export function getArgs() {
    let { source, target, withmd, underline, help } = argv
    if (!source || !target) {
        source = source ?? './src/pages-with-routes'
        target = target ?? './src/routes/meta.ts'
        warnLogImpl(`default: source =====> ${source} , target =====> ${target}`)
    }
    if (help) {
        successLogImpl(USAGE)
    }
    return { source, target, withmd, underline }
}


export function convertUnderlineType(item: string) {
    return item.split('/').filter(_ => !['', '.'].includes(_)).join('_')
}

export function warnLogImpl(txt: string) {
    warnLog(txt, who)
}
export function successLogImpl(txt: string) {
    successLog(txt, who)
}


export function findRouteEntry({ sourcePath = "", prefix = PREFIX, maxLevel = MAX_LEVEL, entryPath = ENTRYPATH, excludePath = EXCLUDE_PATH }) {

    if (!sourcePath) return []
    const rawData: string[][] = []
    function finds(dir = '', parentPath: string[] = []) {
        const dirs = readdirSync(dir)

        dirs.forEach(dirEntry => {
            const _path = resolve(dir, dirEntry)
            const structStat = statSync(_path)
            if (!structStat) return

            if (structStat.isDirectory()) {
                if (dirEntry.startsWith('.') || excludePath.includes(dirEntry)) return

                finds(_path, parentPath.concat(dirEntry))
            } else {
                entryPath.includes(dirEntry) && rawData.push([...parentPath, dirEntry])
            }
        })
    }
    finds(sourcePath, [prefix])
    const data = rawData
        .filter(_ => _.length < maxLevel)
        .sort((a, b) => a[0].localeCompare(b[0]))
        .map(_ => _.join('/'))
        .filter(routePath => !!routePath)
    return data
}

