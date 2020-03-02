use std::fmt::Display;

use collector::collect_modules;

fn main() {
    let things_to_print = collector();

    for thing in things_to_print {
        print(thing);
    }
}

fn print(t: Box<dyn Display>) {
    println!("{}", t);
}

collect_modules!();
