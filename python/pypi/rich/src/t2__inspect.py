from dataclasses import dataclass
from datetime import datetime, UTC

from rich import inspect


@dataclass
class Data:
    a: str
    b: datetime


d1 = Data("asd", datetime.now(UTC))

inspect(d1)
#
# ╭─────────────────────── <class '__main__.Data'> ───────────────────────╮
# │ Data(a: str, b: datetime.datetime)                                    │
# │                                                                       │
# │ ╭───────────────────────────────────────────────────────────────────╮ │
# │ │ Data(a='asd', b=datetime.datetime(2023, 3, 9, 8, 43, 15, 380478)) │ │
# │ ╰───────────────────────────────────────────────────────────────────╯ │
# │                                                                       │
# │ a = 'asd'                                                             │
# │ b = datetime.datetime(2023, 3, 9, 8, 43, 15, 380478)                  │
# ╰───────────────────────────────────────────────────────────────────────╯
