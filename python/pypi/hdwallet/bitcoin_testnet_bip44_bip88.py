from hdwallet import BIP44HDWallet, BIP84HDWallet
from hdwallet.symbols import BTCTEST
from hdwallet.utils import generate_mnemonic

mnemonic = generate_mnemonic()
print("mnemonic:", mnemonic)

print("---- BIP44 ----")

bip44_wallet = BIP44HDWallet(symbol=BTCTEST)
bip44_wallet.from_mnemonic(mnemonic, language="english")
bip44_wallet.clean_derivation()

for address_index in range(10):
    path = f"m/44'/1'/0'/0/{address_index}"
    bip44_wallet.from_path(path=path)
    print(f"{bip44_wallet.path()} / {bip44_wallet.address()} / {bip44_wallet.wif()}")
    bip44_wallet.clean_derivation()

print("---- BIP84 ----")
bip84_wallet = BIP84HDWallet(symbol=BTCTEST)
bip84_wallet.from_mnemonic(mnemonic, language="english")
bip84_wallet.clean_derivation()

for address_index in range(10):
    path = f"m/44'/1'/0'/0/{address_index}"
    bip84_wallet.from_path(path=path)
    print(f"{bip84_wallet.path()} / {bip84_wallet.address()} / {bip84_wallet.wif()}")
    bip84_wallet.clean_derivation()
