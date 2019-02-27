extern crate weave;

#[macro_use]
mod forest;

intersect_tests!(weave::zdd2::Forest<&str>);

union_tests!(weave::zdd2::Forest<&str>);
