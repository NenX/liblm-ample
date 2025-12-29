
import { successLog, warnLog } from 'src/utils';
import argsParse from 'yargs-parser';

const who = 'gen-form-config'


const argv = argsParse(process.argv.slice(2)) as { [x: string]: string }


const USAGE = `usage: [--rm] [--nolazy] [--help] [--target=目标文件夹] [--names=广三,华医]`

export function getArgs() {
    let { rm, help, target, names, nolazy } = argv

    if (help) {
        successLogImpl(USAGE)
        successLogImpl(JSON.stringify({ cwd: process.cwd(), argv: process.argv, names }, null, 2),)

        return null
    }
    return { rm, target, names, nolazy }
}




export function warnLogImpl(txt: string) {
    warnLog(txt, who)
}
export function successLogImpl(txt: string) {
    successLog(txt, who)
}


