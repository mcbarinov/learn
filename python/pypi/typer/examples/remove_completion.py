from typer import Typer
from rich import print

app = Typer(add_completion=False)


@app.command()
def main():
    print("it works")


if __name__ == "__main__":
    app()
