extern crate weave;

#[macro_use]
mod forest;

intersect_tests!(weave::matrix::Forest<&str>);

union_tests!(weave::matrix::Forest<&str>);
