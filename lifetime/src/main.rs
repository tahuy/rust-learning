#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

struct Person {
    name: String
}

struct Company<'l> {
    name: String,
    ceo : &'l Person
}

// impl Person {
//     fn get_ref_name(&self) -> &String {
//         &self.name
//     }
// }

struct People<'l> {
    name: &'l str
}

impl<'l> People<'l> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn lifetime() {
    let boss = Person {name: "Elon Musk".to_string()};
    let company = Company {name: "Tesla".to_string(), ceo: &boss};
    println!("CEO of {} is {}", company.name, company.ceo.name);

    // let mut z: &String;
    // {
    //     let p = Person {name: String::from("John")};
    //     z = p.get_ref_name();
    // }
    // println!("z = {}", z);

    let people = People {name: "Jack"};
    // println!("{}", people.name);
    people.talk();
}

fn main() {
    lifetime();
}
