import time

from rich.progress import Progress, TextColumn, BarColumn, MofNCompleteColumn

progress = Progress(
    TextColumn("[progress.description]{task.description}"),
    BarColumn(),
    MofNCompleteColumn(),
)

with progress:
    task = progress.add_task("[red]Downloading...", total=100)

    for i in range(100):
        progress.update(task, advance=1)
        time.sleep(0.02)
