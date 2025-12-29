import argsParse from 'yargs-parser';
import { colorLog } from 'src/utils'

const argv = argsParse(process.argv.slice(2))
const { _, color, who } = argv

function main() {
    colorLog(_.join(' '), who, color)
}
main()