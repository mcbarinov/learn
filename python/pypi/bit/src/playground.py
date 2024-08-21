import os

from bit import PrivateKeyTestnet
from dotenv import load_dotenv

load_dotenv()

WIF = os.getenv("WIF")
ADDRESS_1 = os.getenv("ADDRESS_1")

key = PrivateKeyTestnet(wif=WIF)
print(key.get_unspents())
