import os

from bit import PrivateKeyTestnet
from dotenv import load_dotenv

load_dotenv()

WIF = os.getenv("WIF")
ADDRESS_1 = os.getenv("ADDRESS_1")

key = PrivateKeyTestnet(wif=WIF)
print("address", key.address)
balance = key.get_balance()
print(balance, type(balance))

tx = key.create_transaction([(ADDRESS_1, 123, "satoshi")])
print(tx)

tx_id = key.send([(ADDRESS_1, 123, "satoshi")])
print(tx_id)
