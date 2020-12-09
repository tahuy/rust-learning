#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

use std::collections::HashMap;
use std::collections::HashSet;

// use std::thread;
// use std::time;

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    // a.push("abc".parse().unwrap());
    // println!("a = {:?}", a);
    let idx: usize = 0;
    a[idx] = 132;
    println!("a[0] = {}", a[idx]);

    a.push(4);

    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None          => println!("error, no such element")
    }

    for x in &a
    {
        println!("x = {}", x);
    }

    a.push(77);

    let last_element = a.pop();
    println!("last element = {:?}, a = {:?}", last_element, a);

    // this was wrong because a.pop can return Some(x) or None
    // let Some(x) = a.pop();

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}

fn hashmap()
{
    let mut shapes = HashMap::new();
    shapes.insert("triangle", 3);
    shapes.insert("square", 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (k, v) in &shapes {
        println!("{}: has {} sides", k, v);
        // println!("{:?}", (k, v));
    }
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    shapes.entry("rectangle".into()).or_insert(2);
    shapes.entry("square".into()).or_insert(4);
    // shapes.insert("square", 4);
    println!("{:?}", shapes);
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

fn hashset()
{
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega
    {
        println!("we added vega");
    }
    if !greeks.contains("kappa")
    {
        println!("we don't have kappa!");
    }
    let removed = greeks.remove("delta");
    if removed
    {
        println!("we removed delta");
    }
    println!("{:?}", greeks);

    let a: HashSet<_> = (1..=5).collect();
    let b: HashSet<_> = (6..=10).collect();
    let c: HashSet<_> = (1..=10).collect();
    let d: HashSet<_> = (2..=8).collect();

    println!("is {:?} a subset of {:?} ?? {}",
             d, c, d.is_subset(&c));
    println!("is {:?} a subset of {:?} ?? {}",
             a, b, a.is_subset(&b));

    // disjoint = no common elements
    println!("is {:?} disjoint with {:?} ?? {}",
             a, b, a.is_disjoint(&b));

    // union
    println!("items in either {:?} and {:?} are {:?}",
             d, b, d.union(&b));
    // intersection (inner join)
    println!("items intersection {:?} and {:?} are {:?}",
             d, b, d.intersection(&b));

    // difference (left join)
    println!("items difference {:?} and {:?} are {:?}",
             d, b, d.difference(&b));
    // symmetric difference = union - intersection (outer join)
    println!("items symmetric difference {:?} and {:?} are {:?}",
             d, b, d.symmetric_difference(&b));
}

fn iterators()
{
    let mut vec = vec![3, 2, 1];
    for x in &vec
    {
        println!("{}", x);
    }

    for x in vec.iter_mut()
    {
        *x += 2;
    }
    println!("{:?}", vec);

    for x in vec.iter().rev()
    {
        println!("x = {}", x);
    }

    let mut vec2 = vec![1, 2, 3];
    // into_iter() - move operation that transforms the collection into a by-value iterator
    //               not the same as ordinary iteration!
    //               useful when you need values but not the collection itself
    // extend() - automatically calls into_iter() to move elements from one collection to another
    vec2.extend(vec);
    println!("vec2 = {:?}", vec2);
    // cannot use vec anymore because it was moved to vec2
    // println!("vec = {:?}", vec);
}

fn main() {
    vectors();
    hashmap();
    hashset();
    iterators();
}
