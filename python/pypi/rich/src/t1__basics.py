from datetime import datetime, UTC

from rich import print as rich_print


rich_print("qwe")
rich_print(123, ["a", True], "ewq", datetime.now(UTC), {"a": 1})
