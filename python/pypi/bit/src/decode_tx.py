import os
from pprint import pprint

from bit.transaction import deserialize
from dotenv import load_dotenv

load_dotenv()

tx_hex = os.getenv("TX_HEX")

res = deserialize(tx_hex)
pprint(res)
