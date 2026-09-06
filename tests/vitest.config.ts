import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    // vitest 2 replaced `threads: false` with a pool selection. One forked
    // process at a time, so the conductors in each scenario do not contend.
    pool: 'forks',
    poolOptions: { forks: { singleFork: true } },
    testTimeout: 60*1000*3 // 3  mins
  },
})
