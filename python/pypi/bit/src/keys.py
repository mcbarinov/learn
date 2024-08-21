from bit import PrivateKey, PrivateKeyTestnet, MultiSig

mainnet_key = PrivateKey()
print("mainnet")
print("address", mainnet_key.address)
print("segwit_address", mainnet_key.segwit_address)
print("wif", mainnet_key.to_wif())
print("public_key", mainnet_key.public_key.hex())


print("\ntestnet")
testnet_key = PrivateKeyTestnet()
print("address", testnet_key.address)
print("segwit_address", testnet_key.segwit_address)
print("wif", testnet_key.to_wif())
print("public_key", testnet_key.public_key.hex())

print("\nmultisig")
key1 = PrivateKey()
key2 = PrivateKey()
multisig = MultiSig(key1, {key1.public_key, key2.public_key}, 2)
print("address", multisig.address)

