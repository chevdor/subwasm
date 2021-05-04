#!/usr/bin/env python
import os

dir = os.path.dirname(os.path.abspath(__file__))

try:
    os.mkdir("out")
except:
    pass

# You can find more at https://github.com/polkadot-js/apps/blob/master/packages/apps-config/src/endpoints/production.ts
# A list of 'some' nodes
nodes = {
    'westend': 'wss://westend-rpc.polkadot.io',
    'kusama': 'wss://kusama-rpc.polkadot.io',
    'polkadot': 'wss://rpc.polkadot.io',
    'centrifuge': 'wss://fullnode.centrifuge.io',
    'darwinia-crab': 'wss://crab-rpc.darwinia.network',
    'darwinia': 'wss://rpc.darwinia.network',
    'subsocial': 'wss://rpc.subsocial.network',
    'plasm': 'wss://rpc.plasmnet.io/',
}

for name in nodes:
    url = nodes[name]
    print(name, url)
    os.system(
        'subwasm get --url {url} -o out/{out}'.format(url=url, out=name + "_runtime.wasm"))
