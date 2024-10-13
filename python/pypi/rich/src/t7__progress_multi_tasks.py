import time

from rich.progress import Progress, TextColumn, BarColumn, MofNCompleteColumn

progress = Progress(
    TextColumn("[progress.description]{task.description}"),
    BarColumn(),
    MofNCompleteColumn(),
)

with progress:
    task1 = progress.add_task("[red]Downloading...", total=100)
    task2 = progress.add_task("[red]Calculation...", total=100)
    task3 = progress.add_task("[red]Esasing...", total=100)

    for i in range(100):
        progress.update(task1, advance=1)
        progress.update(task2, advance=2)
        progress.update(task3, advance=1)
        time.sleep(0.02)
