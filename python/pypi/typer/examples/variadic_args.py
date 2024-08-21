from rich import print
from typer import Typer

app = Typer()


@app.command()
def main(args: list[str]):
    for a in args:
        print(f"arg: {a}")


if __name__ == "__main__":
    app()
