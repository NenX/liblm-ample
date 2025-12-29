const fs = require('fs')
const path = require('path')
function findEntries(base_path) {
    const exists = fs.existsSync(base_path)
    if (!exists) {
        console.error(`${base_path} 不存在！！`)
        return
    }
    const stat = fs.statSync(base_path)
    if (!stat.isDirectory()) {
        console.error(`${base_path} 不是文件夹！`)
        return
    }
    const assertNames = fs.readdirSync(base_path)

    return assertNames
}
function relative_asserts(p) {
    return path.relative('./asserts', p)
}
function getInfo() {
    const assertsPath = path.resolve(__dirname, 'asserts')
    const entries = findEntries(assertsPath)
    if (!assertsPath) return {}
    const metaInfo = { entries }
    const all_files = {}
    entries.forEach(ent => {
        const ent_files = all_files[ent] = {}
        const info = { dirs: {}, files: {} }
        const entPath = path.resolve(assertsPath, ent)

        const typeStat = fs.statSync(entPath)
        const is_dir = typeStat.isDirectory()
        if (!is_dir) {
            return
        }

        const items = findEntries(entPath) ?? []
        items.forEach(i => {
            const iPath = path.resolve(entPath, i)
            const stat = fs.statSync(iPath)

            if (stat.isDirectory()) {
                info.dirs[i] = i
                let sub_files = ent_files[i] = {}

                const sub = findEntries(iPath)
                sub.forEach(s => {
                    const sPath = path.resolve(iPath, s)

                    const s_stat = fs.statSync(sPath)

                    sub_files[s] = relative_asserts(sPath) + (s_stat.isDirectory() ? '/' : '')

                })


            } else {
                ent_files[i] = relative_asserts(iPath)
                info.files[i] = i
            }

        })
        metaInfo[`${ent}_info`] = info

    })
    metaInfo['all_files'] = all_files
    return metaInfo
}
function main() {
    const info = getInfo()
    const filePath = path.resolve(__dirname, 'src/index.ts')
    const keys = Object.keys(info)
    const text = keys.reduce((output, k) => {
        const strArr = info[k]
        return output + `export const ${k} = ${JSON.stringify(strArr, null, 4)} ;\r\n`
    }, '')
    fs.writeFileSync(filePath, text
    )
}

main()