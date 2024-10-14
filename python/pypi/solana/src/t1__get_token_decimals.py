from solana.rpc.api import Client
from solders.pubkey import Pubkey

node = "https://api.mainnet-beta.solana.com"
token_mint_address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"  # USDC

client = Client(endpoint=node)

# https://solana.com/docs/rpc/http/gettokensupply
res = client.get_token_supply(Pubkey.from_string(token_mint_address))

decimals = res.value.decimals

print(decimals, type(decimals))
# 6 <class 'int'>
