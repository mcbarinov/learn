from click.exceptions import Exit
from rich import print

__version__ = "0.1.2"

from typer import Typer, Option


def version_callback(value: bool):
    if value:
        print(f"v{__version__}")
        raise Exit()


app = Typer(add_completion=False)


@app.command()
def hello(name: str):
    print(f"Hello {name}")


@app.command()
def goodbye(name: str, formal: bool = False):
    if formal:
        print(f"Goodbye Ms. {name}. Have a good day.")
    else:
        print(f"Bye {name}!")


@app.callback()
def main(
        _version: bool = Option(None, "--version", callback=version_callback, is_eager=True)
):
    pass


if __name__ == "__main__":
    app()
