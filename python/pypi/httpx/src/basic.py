import httpx

res = httpx.get("https://httpbin.org/ip")

print(res.json())