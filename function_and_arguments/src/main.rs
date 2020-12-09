#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn print_value(x: i32)
{
    println!("value = {}", x);
}

fn increase(x: &mut i32)
{
    *x += 1;
}

fn product(x: i32, y: i32) -> i32
{
    return x * y;
}

fn divide(x: i32, y: i32) -> i32
{
    x / y
}

fn functions()
{
    let x = 45;
    print_value(x);

    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    println!("x * y = {}", product(6, 3));
    println!("x / y = {}", divide(6, 3));
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end  : Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn methods()
{
    let p1 = Point {x: 3.2, y: 7.5};
    let p2 = Point {x: 9.2, y: 1.8};
    let line = Line {start: p1, end: p2};
    println!("length of line = {}", line.len());
}

fn say_hello()
{
    println!("Hello, World!");
}

fn closures()
{
    let action = say_hello;
    action();

    let plus_one = |x: i32| -> i32 {x + 1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }

    let borrow_two = &mut two;
    println!("two is {}", borrow_two);

    let plus_three = |x: &mut i32| {
        *x += 3;
        *x
    };

    let mut f = 5;
    println!("f = {}", plus_three(&mut f));

    let borrow_three = &f - 5i32;
    println!("three is {}", borrow_three);

    let plus_four = |mut x: i32| {
        x += 4;
        x
    };
    let g = 11;
    plus_four(g);
    println!("g = {}", g);
}

fn is_even(x: u32) -> bool
{
    x % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool
{
    move |y| y > limit
}

fn higher_order_function()
{
    let limit = 500;
    let mut sum = 0;
    let about_limit = greater_than(limit);
    for i in 0.. {
        let x = i * i;
        if about_limit(x) {
            break;
        }
        else {
            if is_even(x) {
                sum += x;
            }
        }
    }
    println!("loop sum = {}", sum);

    let sum2 = (0..)
                    .map(|x: u32| x * x) // take value and transform to some other value
                    .take_while(|&x| x < limit) //
                    .filter(|x: &u32| is_even(*x))
                    .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);
}

fn main() {
    functions();
    methods();
    closures();
    higher_order_function();
}
