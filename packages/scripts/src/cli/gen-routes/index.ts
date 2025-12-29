
import { existsSync, writeFileSync } from 'fs';
import { dirname, relative, resolve } from 'path';
import { tplMd } from '../../utils';
import { ANNOTATION, convertUnderlineType, findRouteEntry, getArgs, successLogImpl, warnLogImpl } from './utils';
const args = getArgs()







function main() {
    let { source, withmd } = args

    const sourcePath = resolve(process.cwd(), source)
    if (!existsSync(sourcePath)) return warnLogImpl(`${sourcePath} 不存在`)
    const data = findRouteEntry({ sourcePath })
    writeMeta(data)
    if (withmd) {
        writeMd(data)
    }
    successLogImpl('路由生成✅',)
}


function rawTemplate(data: string[] = []) {
    const str = JSON.stringify(data, null, 2)

    const { source, target, underline } = args

    let relativePath = relative(dirname(target), source).replace('\\', '/')
    relativePath = relativePath.startsWith('.') ? relativePath : `./${relativePath}`
    const staticImport: string[] = []
    const _data = data.reduce((a, b) => {
        const isLazy = b.includes('index.ts')
        const routePath = b.replace(/\/index(.nl)??.tsx/, '').toLocaleLowerCase()
        const keyName = underline ? convertUnderlineType(routePath) : routePath
        const rawPath = `"${relativePath}${b.replace(/\.tsx/, '')}"`
        const moduleName = routePath.slice(routePath.lastIndexOf('/') + 1).split('-').map(_ => _.toUpperCase()).join('_')
        if (!isLazy) staticImport.push(`import ${moduleName} from ${rawPath}\n`)


        const C = isLazy ? `lazy(() => import(${rawPath}))` : `${moduleName}`
        const row = `    "${keyName}":${C},\n`
        return a + row
    }, '')
    const importStr = staticImport.reduce((a, b) => a + b, '')
    return `
${ANNOTATION}
import { FC, lazy, LazyExoticComponent, ComponentType } from 'react'; //
${importStr}
export type TC = LazyExoticComponent<ComponentType> | FC

export const routesData = {
${_data}}`
}

function writeMeta(data: string[] = []) {
    const target = args.target
    const targetPath = resolve(process.cwd(), target)

    const buf = rawTemplate(data)
    writeFileSync(targetPath, buf)
}
function writeMd(data: string[] = []) {
    const source = args.source || ''

    const rowsData = data.reduce((a, b) => {
        const dir = b.slice(0, b.lastIndexOf('/')).toLowerCase()
        const infoPath = resolve(`${source}${dir}`, 'info.json')
        let info = { name: '缺省' }
        try {
            info = require(infoPath)
        } catch (error) {

        }
        return [...a, [dir, info.name]]
    }, [] as string[][])
    const tableData = {
        table: {
            headers: ['路径', '说明'],
            rows: rowsData
        }
    }
    tplMd(data.length ? tableData : null,)

}




main()
