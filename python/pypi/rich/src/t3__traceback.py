from rich.traceback import install

# install(show_locals=True)
install(show_locals=False)


def fail(a):
    a = a + 1
    return a / 0


fail(10)
