#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

struct SomeOptions2 {
    foo: i32,
    bar: f32,
}

//How can we define some default values? You can use Default:


impl Default for SomeOptions2 {
    fn default() -> SomeOptions2 {
        SomeOptions2 { foo: 0, bar: 0.0 }
    }
}

impl Drop for SomeOptions2 {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Example {
    number: i32,
}

impl Example {
    fn boo() {
        println!("boo! Example::boo() was called!");
    }

    fn answer(&mut self) {
        self.number += 42;
    }

    fn get_number(&self) -> i32 {
        self.number
    }
}

//This trait example is not used in the tutorial

trait Thingy {
    fn do_thingy(&self);
}

impl Thingy for Example {
    fn do_thingy(&self) {
        println!("doing a thing! also, number is {}!", self.number);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn main() {
    let _options: SomeOptions = Default::default();
    let _options2 = SomeOptions2 {
        foo: 42,
        ..Default::default()
    };

    let _origin = Point { x: 0, y: 0 };

    
    //This code is used to create the unimplemented error for the tutorial
    //println!("Struct Print Test {}",_options)
}
