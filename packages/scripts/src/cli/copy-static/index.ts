
import fs from 'fs';
import path from 'path';
import argsParse from 'yargs-parser';
import { findEntries, successLogImpl, warnLogImpl } from './utils';

const argv = argsParse(process.argv.slice(2))
const { target = '/public', silent = false } = argv

const static_path = path.resolve(process.cwd() + '/node_modules/@lm_fe/static/asserts')
const public_path = path.resolve(process.cwd() + target)


function doit() {
    const staticAssertNames = findEntries(static_path)
    if (!staticAssertNames)
        return warnLogImpl(`解析${static_path} 出错！`)

    const publicAssertNames = findEntries(public_path, true)
    if (!publicAssertNames)
        return warnLogImpl(`解析${public_path} 出错！`)



    staticAssertNames.forEach(n => {
        const srcDirPath = path.resolve(static_path, n)
        const dstDirPath = path.resolve(public_path, n)
        if (publicAssertNames.includes(n))
            return warnLogImpl(`${dstDirPath} 已存在！`)

        fs.cp(srcDirPath, dstDirPath, { recursive: true }, err => {
            if (err) {
                warnLogImpl(`cp 出错: ${err}`,)
            } else {
                successLogImpl(`复制 ${dstDirPath} 成功！`)
            }
        })
    })
}


function main() {
    doit()
}

main()