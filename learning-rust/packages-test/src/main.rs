
use utils::hosting::add_to_waitlist;
mod utils;

use garden::vegetables::{Asparagus, toto_call};
pub mod garden;

fn main() {
    let plant = Asparagus {};

    // eat_at_restaurant();
    dbg!("Hello, world! {}", plant);
    toto_call();

    add_to_waitlist();
    
}
