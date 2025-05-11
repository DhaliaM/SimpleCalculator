use leptos::*;
use simple_calculator::SimpleCalculator;
use leptos::mount::mount_to_body;

mod simple_calculator;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(SimpleCalculator);
}

