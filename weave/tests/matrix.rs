extern crate weave;

#[macro_use]
mod forest;

#[cfg(test)]
mod matrix_tests {
    intersect_tests!(weave::matrix::Forest<&str>);

    union_tests!(weave::matrix::Forest<&str>);

    product_tests!(weave::matrix::Forest<&str>);

    subset_tests!(weave::matrix::Forest<&str>);
}