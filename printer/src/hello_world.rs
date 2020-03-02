use std::fmt::Display;

fn hello_world_helper() -> impl Display {
    "Hello, World"
}

pub fn unsure() -> impl Display {
    format!("{}???", hello_world_helper())
}

pub fn excited() -> impl Display {
    format!("{}!!!", hello_world_helper())
}
