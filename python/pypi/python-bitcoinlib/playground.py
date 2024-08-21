import os

from bitcoin.core import CTransaction
from bitcoin.core import x
from bitcoin.wallet import CBitcoinAddress
from dotenv import load_dotenv


def decode_transaction_hex(tx_hex):
    # Convert hex string to raw bytes
    tx_bytes = x(tx_hex)

    # Deserialize the transaction
    tx = CTransaction.deserialize(tx_bytes)
    print(tx)

    # Print basic transaction information
    print(f"Transaction ID: {tx.GetTxid()}")
    print(f"Version: {tx.nVersion}")
    print(f"Locktime: {tx.nLockTime}")
    #
    # Decode inputs
    print("\nInputs:")
    for i, tx_in in enumerate(tx.vin):
        print(f"  Input {i + 1}:")
        print(f"    Previous TXID: {tx_in.prevout.hash}")
        print(f"    Output Index: {tx_in.prevout.n}")
        print(f"    ScriptSig: {tx_in.scriptSig}")
        print(f"    Sequence: {tx_in.nSequence}")

    # Decode outputs
    print("\nOutputs:")
    for j, tx_out in enumerate(tx.vout):
        print(f"  Output {j + 1}:")
        print(f"    Value (satoshis): {tx_out.nValue}")
        print(f"    ScriptPubKey: {tx_out.scriptPubKey}")

        # Attempt to extract address if possible
        try:
            address = CBitcoinAddress.from_scriptPubKey(tx_out.scriptPubKey)
            print(f"    Address: {address}")
        except Exception as e:
            print(f"    Address: Unable to decode ({str(e)})")


# Example transaction hex (replace with actual transaction hex)



load_dotenv()
tx_hex = os.getenv("TX_HEX")
# Decode the transaction
decode_transaction_hex(tx_hex)
