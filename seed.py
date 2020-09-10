#!/bin/python

from mnemonic import Mnemonic
from monero.seed import Seed
import kstretch
import sys


seed_hex = kstretch.process(sys.argv[1])
monero_seed = Seed(seed_hex)
print("monero seed: " + str(monero_seed.phrase))

bip39 = Mnemonic("english")
bip39_seed = bip39.to_mnemonic(bytes.fromhex(seed_hex))

print("bip39 seed: " + bip39_seed)

bip39_12 = Mnemonic("english")
bip39_seed_12 = bip39.to_mnemonic(bytes.fromhex(seed_hex[0:len(seed_hex) // 2]))

print("bip39 seed 12: " + bip39_seed_12)



