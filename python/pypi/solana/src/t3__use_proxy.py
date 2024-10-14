import os

import httpx
from dotenv import load_dotenv
from httpx_socks import SyncProxyTransport
from solana.rpc.api import Client
from solana.rpc.commitment import Commitment
from solana.rpc.providers.core import _after_request_unparsed  # noqa
from solana.rpc.providers.http import HTTPProvider
from solders.rpc.requests import Body


class ProxyClient(Client):
    def __init__(
            self,
            endpoint: str | None = None,
            commitment: Commitment | None = None,
            timeout: float = 10,
            extra_headers: dict[str, str] | None = None,
            *,
            proxy: str,
            **kwargs
    ):
        """Init API client."""
        super().__init__(commitment)
        self._provider = ProxyHTTPProvider(
            endpoint=endpoint,
            timeout=timeout,
            extra_headers=extra_headers,
            proxy=proxy,
            **kwargs)

    def __enter__(self):
        self._provider.__enter__()
        return self

    def __exit__(self, *args):
        self._provider.__exit__(*args)

    def close(self):
        self._provider.close()


class ProxyHTTPProvider(HTTPProvider):
    def __init__(
            self,
            endpoint: str | None = None,
            extra_headers: dict[str, str] | None = None,
            timeout: float = 10,
            *,
            proxy: str | None = None,
            **kwargs
    ):
        super().__init__(
            endpoint=endpoint,
            extra_headers=extra_headers,
            **kwargs
        )
        self.session = Session(proxy, timeout=timeout)
        self._proxy = proxy

    def __str__(self) -> str:
        return f"HTTP RPC connection {self.endpoint_uri} | proxy={self._proxy}"

    def make_request_unparsed(self, body: Body) -> str:
        request_kwargs = self._before_request(body=body)
        raw_response = self.session.post(**request_kwargs)
        return _after_request_unparsed(raw_response)

    def make_batch_request_unparsed(self, reqs: tuple[Body, ...]) -> str:
        """Make an async HTTP request to an http rpc endpoint."""
        request_kwargs = self._before_batch_request(reqs)
        raw_response = self.session.post(**request_kwargs)
        return _after_request_unparsed(raw_response)

    def is_connected(self) -> bool:
        """Health check."""
        try:
            response = self.session.get(self.health_uri)
            response.raise_for_status()
        except (IOError, httpx.HTTPError) as err:
            self.logger.error("Health check failed with error: %s", str(err))
            return False

        return response.status_code == httpx.codes.OK

    def __enter__(self):
        self.session.__enter__()
        return self

    def __exit__(self, *args):
        self.session.__exit__(*args)

    def close(self):
        self.session.close()


class Session(httpx.Client):
    def __init__(self, proxy: str | None = None, follow_redirects=True, **kwargs):
        if proxy:
            if proxy.startswith("http"):
                kwargs['proxy'] = proxy
            elif proxy.startswith("socks"):
                kwargs['transport'] = SyncProxyTransport.from_url(proxy)
        super().__init__(follow_redirects=follow_redirects, **kwargs)




load_dotenv()



client = ProxyClient(endpoint="https://api.mainnet-beta.solana.com", proxy=os.getenv("PROXY"))
with client:
    print(client.get_block_height())
