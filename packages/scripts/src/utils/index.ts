import { writeFileSync } from 'fs';
import json2md from 'json2md';
import { resolve } from 'path';
import dayjs from 'dayjs'

import chalk from 'chalk';





const colorFnMap = {
    green: chalk.green,
    bgGreen: chalk.bgGreen,
    yellow: chalk.yellow,
    bgYellow: chalk.bgYellow,
    red: chalk.red,
    bgRed: chalk.bgRed,
    blue: chalk.blue,
    bgBlue: chalk.bgBlue,
    cyan: chalk.cyan,
    bgCyan: chalk.bgCyan,
}

export function colorLog(str = '', who = '', color: keyof typeof colorFnMap = 'bgCyan') {
    const whoPadding = who ? `${who}: ` : ''
    const fn = colorFnMap[color]
    if (!fn) return console.info(chalk.red(`color ${color} 不存在！`))
    const time = dayjs().format('YYYY-MM-DD HH:mm:ss')
    console.info(fn(`[${time}] ${whoPadding}${str}`));
}

export function warnLog(txt: string, who: string) {
    colorLog(txt, who, 'yellow')
}
export function successLog(txt: string, who: string) {
    colorLog(txt, who, 'green')
}

export function tplMd(data: any, mdName = 'README.md', tplName = 'md.json',) {

    const targetPath = resolve(process.cwd(), mdName)

    const tplPath = resolve(process.cwd(), tplName)

    const tpl = require(tplPath) as any[]
    const index = tpl.findIndex(_ => _ === 'slot')
    tpl.splice(index, 1, data,)

    const buf = json2md(tpl.filter(_ => !!_))

    writeFileSync(targetPath, buf)

}