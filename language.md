# The Marietta language

Marietta is a low level language that is easy to use like Python.

Marietta uses a rust memory model with immutable and mutable references.

## types

u8-u2048 unsigned integer
i8-i2048 signed integer
f8-f128 float (with a _n suffix for mantissa size eg. f16_7.)
(TBD) bit-sized float and integer types.

### struct

struct Fred:
    x: u64
    y: u64

x = Fred(x=1, y=2)

### enum

enum George:
    Arm(u64)
    Leg:
        knee: u64
    Hat

x = George::Leg(knee=2)

### union

union Harry:
    x: u64
    y: f64

### References

var y : &u64 = &1
var z : &mut u64 = &2

### Pointers

var p : * const u64
var q : * mut u64

### Reference rules are like Rust, ie. no immutable / mutable mixture and only one mutable.

trait ToString:
    type X;
    def to_string() -> String

### Ranges

    a = 0..10

### Slices

var x : &[u64] = &[1, 2, 3]
var y = &x[0..1]

A fat pointer with ptr/len

### Multislices

var x : &[[u64]] = &[[1, 0], [0, 1]]

### Arrays/Tensors

var x : [u64; 3] = [1, 2, 3]
var y : [[u64; 2]; 2] = [[1, 0], [0, 1]]

### Generics (no phantomdata needed)

struct Fred<T>:
    x: T

def fred<T>() -> T:
    "hello"

trait Jim<T>:
    def z(t: T)

### Lifetimes

struct Fred<'a>:
    x: &'a str

Same rules as rust including elision.

### Strings

let x : &str = "hello"

