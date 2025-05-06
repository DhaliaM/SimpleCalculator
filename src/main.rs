use leptos::*;
use simple_calculator::SimpleCalculator;
use leptos::mount::mount_to_body;

mod simple_calculator;

fn main() {
    mount_to_body(SimpleCalculator);
}

