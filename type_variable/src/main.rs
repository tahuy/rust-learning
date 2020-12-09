#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

use std::mem;
// Global variable
const MEANING_OF_LIFE:i8 = 42; // no fixed address
static K:i32 = 35;
static mut H:i32 = 48;
fn type_variable()
{
    println!("Function type_variable!");
    let x: i32 = 2;
    println!("x = {}", x);

    // x = 234;
    let mut y: i8 = 0;
    println!("y = {} before", y);
    y = 15;
    println!("y = {} after", y);
    let z = 123456789;
    println!("z = {}, takes up {} bytes", z, mem::size_of_val(&z));

    // usize and isize
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let m: char = 'x';
    println!("{} is a char, size = {} bytes", m, mem::size_of_val(&m));

    let e = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));
}

fn operators()
{
    println!("\n\nFunction operators!");
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    a -= 2;
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    // | OR & AND ^ XOR ! NOR
    let c = 1 | 2;
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);
    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("{}", pi_less_4);
    let x = 5;
    let _x_is_5 = x == 5;
    println!("{}", _x_is_5);
}

fn scope_and_shadowing()
{
    println!("\n\nFunction scope and shadowing!");
    let a = 123;
    {
        let a = 456;
        println!("{}", a);
    }
    println!("{}", a)
}

fn declare_and_constant()
{
    println!("\n\nFunction declare and constant!");
    println!("MEANING_OF_LIFE = {}", MEANING_OF_LIFE);
    println!("K = {}", K);

    // unsafe to notice work carefully with global mut variable
    unsafe
    {
        H = 99;
        println!("H = {}", H);
    }
}

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

fn stack_and_heap()
{
    println!("\n\nFunction stack and heap!");
    let x = 5;
    println!("x = {}", x);
    let y = Box::new(44);
    println!("y = {}", *y);

    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p3 takes up {} bytes", mem::size_of_val(&p3));
}

fn main() {
    type_variable();
    operators();
    scope_and_shadowing();
    declare_and_constant();
    stack_and_heap();
}
