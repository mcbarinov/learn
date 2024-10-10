import time

from rich.console import Console
from rich.live import Live
from rich.table import Table

t = Table()
t.add_column("A")
t.add_column("B")

with Live(t, refresh_per_second=5, transient=True, screen=True):
    for i in range(5):
        t.add_row(f"a_{i}", f"b_{i}")
        time.sleep(2)

c = Console()
c.print("done")