def foo() -> u64:
    return 42

def bar() -> u64:
    return foo()

def main():
    var x: u64 = bar()
    pass
