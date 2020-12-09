#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

use std::fmt::Debug;


trait Animal {
    // fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Cat {
    name: &'static str
}

struct Human {
    name: &'static str
}

impl Animal for Cat {
    // fn create(name: &'static str) -> Cat {
    //     return Cat {name};
    // }
    fn name(&self) -> &'static str
    {
        return self.name;
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

impl Animal for Human {
    // fn create(name: &'static str) -> Human {
    //     return Human {name};
    // }
    fn name(&self) -> &'static str
    {
        return self.name;
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

trait Summable <T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result = 0;
        for x in self {
            result += *x;
        }
        return result;
    }
}

fn traits()
{
    let human = Human {name: "John"};
    // let human = Human::create("John");
    // let human:Human = Animal::create("John");
    human.talk();

    let cat = Cat {name: "Kitty"};
    cat.talk();

    let a: Vec<i32> = vec![1, 2, 3];
    println!("sum of vector = {}", a.sum());
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }
}

// fn print_info(shape: impl Shape)
// fn print_info(shape: impl Shape + Debug)

// This is useful when we have multi params (shape: T, shape2: T, ...)
// fn print_info<T: Shape + Debug>(shape: T)

fn print_info<T>(shape: T)
    where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area is: {}", shape.area());
}

fn trait_parameters()
{
    let c = Circle {radius: 4.0};
    print_info(c);
}

struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person {name: name.to_string()}
    // }
    // fn new<S: Into<String>>(name: S) -> Person
    fn new<S>(name: S) -> Person
        where S: Into<String>
    {
        Person { name: name.into()}
    }

    fn print_name(&self) {
        println!("name is {}", self.name);
    }
    // fn print_name2(&self) {
    //     println!("name is {}", self.name);
    // }

    // After return, ownership of Person.name was removed.
    // So we cannot call other return, example get_name2()
    fn get_name(&self) -> String {
        return self.name.to_string();
    }

    fn get_name2(self) -> String {
        return self.name;
    }
}

fn trait_into()
{
    let s = "hello world";
    println!("{}", s.replace("world", "world!"));
    let john = Person::new("John");
    john.print_name();
    // john.print_name2();
    println!("name is {}", john.get_name());
    println!("name is {}", john.get_name2());
    // Name was moved
    // println!("name is {}", john.name);

    let name: String = "Jane".to_string();
    let _jane = Person::new(name/*.as_ref()*/);
}

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature {name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn drops() {
    let clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        // drop(goblin);
        clever = goblin;
        println!("end of scope");
    }
    println!("{}", clever.name);
}

use std::ops::{Add, AddAssign, Neg};
use std::cmp::{PartialEq};

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
    where T: Neg<Output = T>
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

impl<T> PartialEq for Complex<T>
    where T: PartialEq
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
    fn ne(&self, rhs: &Self) -> bool {
        self.re != rhs.re || self.im != rhs.im
    }
}

fn overload_operators() {
    let mut a = Complex::new(1, 2);
    let b = Complex::new(3, 4);
    println!("{:?}", a);
    println!("{:?}", b);
    // println!("{:?}", a + b);
    a += b;
    println!("{:?}", a);
    // println!("{:?}", -a);
    println!("{:?}", a == a);
}

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

fn print_it_static<T: Printable>(z: T)
// fn print_it_static(z: Printable)
// `dyn Printable` does not have a constant size known at compile-time
{
    println!("{}", z.format());
}

fn static_dispatch() {
    let mut a = 123;
    let b = "hello".to_string();
    // println!("{}", a.format());
    // println!("{}", b.format());

    print_it_static(a);
    print_it_static(b);
    a += 1;
    print_it_static(a);
}

fn print_it_dynamic(z: &dyn Printable) {
    println!("{}", z.format());
}

fn dynamic_dispatch() {
    let a = 123;
    let b = "hello".to_string();
    print_it_dynamic(&a);
    print_it_dynamic(&b);

    let shapes:[&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0}
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("area of [{}] is {}", i, shape.area());
    }
}

enum Mixed {
    Human(Human),
    Cat(Cat)
}

fn vector_different_objects() {
    let mut creatures = Vec::new();

    creatures.push(Mixed::Human(Human {name: "John"}));
    creatures.push(Mixed::Cat(Cat {name: "Luff"}));
    for c in creatures {
        match c {
            Mixed::Human(h) => h.talk(),
            Mixed::Cat(k) => k.talk()
        }
    }

    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human {name: "Steven"}));
    animals.push(Box::new(Cat {name: "Cindy"}));
    for a in animals.iter() {
        a.talk();
    }
}

fn main() {
    traits();
    trait_parameters();
    trait_into();
    drops();
    overload_operators();
    static_dispatch();
    dynamic_dispatch();
    vector_different_objects();
}
