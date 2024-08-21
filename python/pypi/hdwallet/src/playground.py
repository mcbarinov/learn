from hdwallet import HDWallet
from hdwallet.symbols import BTCTEST

w = HDWallet(symbol=BTCTEST)
w.from_wif("cVsAqVh6UtxisRLEstrKczmiBahuK4cbymDjX2rUnDAVewidhYtd")

print(w)


