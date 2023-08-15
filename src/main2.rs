use tikv_jemallocator::Jemalloc;

#[global_allocator]
static A2: Jemalloc = Jemalloc;

fn main() {
    println!("Hello, world!");
}
