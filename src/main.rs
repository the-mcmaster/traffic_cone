#![feature(never_type)]

use lib::app::{ARGS, handle_mode};

#[allow(unused_variables)]
fn main() -> ! {
    handle_mode(ARGS.mode().clone())
}
