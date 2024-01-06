import test from 'ava'

import { add } from '../index.js'

test('add from native', (t) => {
  t.is(add(1, 2), 3)
})
