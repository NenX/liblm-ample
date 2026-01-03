const x = require('@lm_fe/lm_x')
x.cli(process.argv.slice(1)).then(x => console.log({ x }))

