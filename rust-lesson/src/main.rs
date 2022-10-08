extern crate lib_demo;
mod debug;
mod enums;
mod error_handling;
mod generics;
mod lifetime;
mod ownership;
mod stack_heap;
mod structs;
mod traits;
mod unit_tests;
mod vars;

fn main() {
    debug::run();
    vars::run();
    stack_heap::run();
    ownership::run();
    generics::run();
    lifetime::run();
    structs::run();
    enums::run();
    traits::run();
    error_handling::run();
    lib_demo::print_random_number();
}
