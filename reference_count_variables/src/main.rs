#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

use std::thread;
// Pass variable around
use std::rc::Rc;
// Pass variable between threads and read only
use std::sync::Arc;
// Pass variable between threads and modify
use std::sync::Mutex;

struct PersonRc {
    name: Rc<String>
}

impl PersonRc {
    fn new(name: Rc<String>) -> PersonRc {
        PersonRc {name}
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn rc_demo() {
    // Usual RC
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = PersonRc::new(name.clone());
        person.greet();
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
}

struct PersonArc {
    name: Arc<String>
}

impl PersonArc {
    fn new(name: Arc<String>) -> PersonArc {
        PersonArc {name}
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn arc_demo() {
    // Atomic RC
    let name = Arc::new("John".to_string());
    let person = PersonArc::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("name = {}", name);
    t.join().unwrap();
}

struct PersonMutex {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl PersonMutex {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> PersonMutex {
        PersonMutex {name, state}
    }
    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}

fn mutex_demo() {
    // Mutex ARC
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = PersonMutex::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
    println!("state = {}", state.lock().unwrap().as_str());
}

fn main() {
    rc_demo();
    arc_demo();
    mutex_demo();
}
