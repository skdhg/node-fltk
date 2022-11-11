import test from 'ava'

import { version } from '../index.js'

test('fltk version', (t) => {
  t.is(typeof version(), "string")
})
