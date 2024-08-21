import typer

app = typer.Typer()

group1_app = typer.Typer()
app.add_typer(group1_app, name="group1")

group2_app = typer.Typer()
app.add_typer(group2_app, name="group2")


@group1_app.command()
def c1():
    print("group1: c1 cmd")


@group1_app.command()
def c2():
    print("group1: c2 cmd")


@group2_app.command()
def c3():
    print("group2: c3 cmd")


@group2_app.command()
def c4():
    print("group2: c4 cmd")


if __name__ == "__main__":
    app()
