import test from 'ava'
import { publicKey, rpcUrl } from './const.js'
import { Address, Contract, networks } from 'test-hello-world'

const addr = Address.fromString(publicKey)

const contract = new Contract({
  ...networks.standalone,
  rpcUrl,
  wallet: {
    isConnected: () => Promise.resolve(true),
    isAllowed: () => Promise.resolve(true),
    getUserInfo: () => Promise.resolve({ publicKey }),
    signTransaction: async (tx: string, opts?: {
      network?: string,
      networkPassphrase?: string,
      accountToSign?: string,
    }) => tx,
  },
})

test('hello', async t => {
  t.deepEqual(await contract.hello({ world: 'tests' }), ['Hello', 'tests'])
})

test('auth', async t => {
  t.is(await contract.auth({ addr, world: 'lol' }), addr)
})

test('inc', async t => {
  t.is(await contract.getCount(), 0)
  t.is(await contract.inc(), 1)
  t.is(await contract.getCount(), 1)
})
