import os

from bitcoinlib.transactions import Transaction
from dotenv import load_dotenv

load_dotenv()

tx_hex = os.getenv("TX_HEX")

res = Transaction.parse(tx_hex, network="testnet")
res.verify()
print(res.as_json())

