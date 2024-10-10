from rich.console import Console
from rich.prompt import Prompt

console = Console()

password = Prompt.ask("Your password", password=True)
print(f"password: {password}")
