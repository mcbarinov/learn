import os

import httpx
from dotenv import load_dotenv

load_dotenv()

# socks proxy
SOCKS_PROXY = os.getenv("SOCKS_PROXY")
print("SOCKS_PROXY:", SOCKS_PROXY)
res = httpx.get("https://httpbin.org/ip", proxy=SOCKS_PROXY)
print(res.json())

# http proxy
HTTP_PROXY = os.getenv("HTTP_PROXY")
print("HTTP_PROXY:", HTTP_PROXY)
res = httpx.get("https://httpbin.org/ip", proxy=HTTP_PROXY)
print(res.json())
