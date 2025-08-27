#![feature(never_type)]

use lib::app::ARGS;
use lib::handle::handle_mode;

#[allow(unused_variables)]
fn main() -> ! {
    handle_mode(ARGS.mode().clone())
}
