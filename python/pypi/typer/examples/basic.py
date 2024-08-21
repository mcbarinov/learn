from rich import print
from typer import Typer

app = Typer()


@app.command(help="first command")
def cmd1(name: str):
    print(f"Hello {name}")


@app.command(help="second command")
def cmd2(name: str, formal: bool = False):
    if formal:
        print(f"Goodbye Ms. {name}. Have a good day.")
    else:
        print(f"Bye {name}!")


if __name__ == "__main__":
    app()
