#![recursion_limit = "128"]

extern crate weave;
#[macro_use]
extern crate yew;

use yew::prelude::*;

mod visualizer;

fn main() {
    yew::initialize();
    App::<self::visualizer::Model>::new().mount_to_body();
    yew::run_loop();
}
