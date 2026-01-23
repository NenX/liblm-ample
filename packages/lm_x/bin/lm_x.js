#!/usr/bin/env node
const x = require('@lm_fe/lm_x')
if (process.argv.length > 2) {
  x.cli(process.argv.slice(1))
} else {
  x.prompt()
}

