use std::alloc::System;

#[global_allocator]
static A1: System = System;

fn main() {
    println!("Hello, world!");
}
