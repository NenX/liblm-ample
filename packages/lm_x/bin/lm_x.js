const x = require('@lm_fe/lm_x')
// process.env.RUST_BACKTRACE = "1"
if (process.argv.length > 2) {
  x.cli(process.argv.slice(1))
} else {
  x.prompt()
}

