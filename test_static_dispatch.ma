# Simple test for static method dispatch

struct Point:
    x: u64
    y: u64

trait Drawable:
    def draw(self) -> None:

impl Drawable for Point:
    def draw(self) -> None:
        pass

def main():
    var p: Point = Point { x: 1, y: 2 }
    # This should use static dispatch: direct call to Point__draw(p)
    p.draw()
    pass
