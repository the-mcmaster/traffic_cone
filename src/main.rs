use lib::app::ARGS;
use lib::handle::handle_mode;

fn main() -> ! {
    handle_mode(ARGS.mode().clone())
}
