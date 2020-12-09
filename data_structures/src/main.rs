#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end  : Point
}

fn structures()
{
    let p = Point {x: 3.2, y: 4.0};
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point {x: 5.0, y: 10.0};
    let line = Line {start: p, end: p2};
    println!("line start at ({}, {}), end at ({}, {})",
             line.start.x, line.start.y, line.end.x, line.end.y);
}

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor {cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn enums()
{
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255
    };

    match c {
        Color::Red                  => println!("r"),
        Color::Green                => println!("g"),
        Color::Blue                 => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255 }            => println!("black"),
        Color::RgbColor(_r, _g, _b) => println!("rgb"),
        _                           => println!("some other")
    }
}

union IntOrFloat
{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe
    {
        match iof
        {
            IntOrFloat{ i: 42 } => {
                println!("meaning of life");
            },
            IntOrFloat{ f } => {
                println!("value = {}", f);
            }
        }
    }
}

fn option_t()
{
    let x = 3.0;
    let y = 0.0;
    // Option T -> Some(v) | None (useful for API)
    let result = if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None         => println!("cannot divide by zero")
    }

    // Check Some(z) can be assigned by result or not -> return true or false
    if let Some(z) = result
    {
        println!("result = {}", z);
    }
    else
    {
        println!("cannot divide by zero");
    }
}

fn arrays()
{
    let mut a:[i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("{:?}", a);

    if a != [9, 2, 3, 4, 5]
    {
        println!("does not match");
    }

    let b = [1u16; 10];
    for i in 0..b.len()
    {
        println!("{}", b[i]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32; 3]; 2] =
    [
        [1.0, 0.0, 0.5],
        [0.0, 2.0, 1.5]
    ];
    println!("{:?}", mtx);
    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn use_slice(slice: &mut[i32])
{
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = 50;
}

fn slices()
{
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
    println!("{:?}", data);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32)
{
    (x + y, x * y)
}

fn tuples()
{
    let x= 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // Destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 9);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last element = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;
    println!("c = {}, d = {}, e = {}, f = {}", c, d, e, f);
}

fn how_many(x: i32) -> &'static str
{
    match x {
        0                 => "no",
        1 | 2             => "one or two",
        12                => "a dozen",
        z @ 9...11   => "lots of",
        _ if (x % 2 == 0) => "some",
        _                 => "a few"
    }
}

fn pattern_matching()
{
    for x in 0..13
    {
        println!("{}: I have {} oranges", x, how_many(x));
    }
    let point = (3, 4);
    match point {
        (0, 0)          => println!("origin"),
        (0, y)      => println!("x axis, y = {}", y),
        (x, 0)      => println!("x = {}, y axis", x),
        (x, y) => println!("{}, {}", x, y),
        _                => println!("somewhere else")
    }

    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255
    };

    match c {
        Color::Red                  => println!("r"),
        Color::Green                => println!("g"),
        Color::Blue                 => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            black: 255, ..}         => println!("black"),
        Color::RgbColor(_r, _g, _b) => println!("rgb"),
        _                           => println!("some other")
    }
}

struct Couple<T>
{
    x: T,
    y: T
}

struct Stream<T>
{
    start: Couple<T>,
    end  : Couple<T>
}

fn generics()
{
    let a = Couple {x: 3f32, y: 4f32};
    let b = Couple {x: 1.4, y: 3.5};
    let c = Stream {start: a, end: b};

    println!("({}, {}), ({}, {})", c.start.x, c.start.y, c.end.x, c.end.y);
}

fn main() {
    structures();
    enums();

    let mut iof: IntOrFloat = IntOrFloat {i: 123};
    iof.i = 234;
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat{f: 42.0});
    option_t();
    arrays();
    slices();
    tuples();
    pattern_matching();
    generics();
}
