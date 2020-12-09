#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

struct Student {
    name: String
}

impl Student {
    fn courses(&self, platform: &Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course {
    name: String
}

struct Enrollment<'l> {
    student: &'l Student,
    course : &'l Course
}

impl<'l> Enrollment<'l> {
    fn new(student: &'l Student, course: &'l Course) -> Enrollment<'l> {
        Enrollment {student, course}
    }
}

struct Platform<'l> {
    enrollments: Vec<Enrollment<'l>>
}

impl<'l> Platform<'l> {
    fn new() -> Platform<'l> {
        Platform {enrollments: Vec::new()}
    }

    fn enroll(&mut self, student: &'l Student, course: &'l Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }

    fn print_info(&self) {
        for enroll in self.enrollments.iter() {
            println!("{} enrolls {} course", enroll.student.name, enroll.course.name);
        }
    }
}

fn circular_reference() {
    let john = Student {name: "John".into()};
    let course = Course {name: "Intro to Rust".into()};

    let mut p = Platform::new();
    p.enroll(&john, &course);
    // p.print_info();
    let course1 = Course {name: "Machine Learning".into()};
    p.enroll(&john, &course1);
    p.print_info();

    println!("\n\nCourses enroll by John:");
    for c in john.courses(&p) {
        println!("{}", c);
    }

    let jane = Student {name: "Jane".into()};
    let course2 = Course {name: "Learning how to learn".into()};
    p.enroll(&jane, &course);
    p.enroll(&jane, &course2);
    println!("\n\nCourses enroll by Jane:");
    for c in jane.courses(&p) {
        println!("{}", c);
    }
}

use std::thread;
use std::time;

fn threads() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });
    for _ in 1..10 {
        print!("_");
        thread::sleep(time::Duration::from_millis(300));
    }

    match handle.join() {
        Ok(_)  => println!("\nOk"),
        Err(_) => println!("\nError")
    };
}

fn main() {
    circular_reference();
    threads();
}
