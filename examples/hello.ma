# hello.ma — the classic first program, Marietta style.
#
# Marietta compiles to native code via Cranelift.  The `run` subcommand
# JIT-compiles the file and calls `main()` if one is defined.
#
# Build:   marietta build hello.ma
# Run:     marietta run   hello.ma
#
# Note: Full string literal support is in development.
# For now, demonstrating basic print with integers.

def main():
    var s = "hello, marietta!"
    pass
