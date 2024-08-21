from hdwallet import BIP44HDWallet
from hdwallet.utils import generate_mnemonic

w = BIP44HDWallet()
w.from_mnemonic(generate_mnemonic())
w.clean_derivation()
print(w.private_key(), type(w.private_key()))
print(w.wif(), type(w.wif()))

# 2c8ba83874d8b28efd20fa758dfc971ba9b92f48f088ecdd145f8c561ad2b9ff <class 'str'>
# KxiJSmpyYFh4QPsTkdpo1pZi68pqPuJgLbn4XvRR2deLUDn2dfeC <class 'str'>
