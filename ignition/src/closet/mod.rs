use std::collections::BTreeSet;

use core::Item;
use weave::Tree;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Closet {
    tree: Tree<Item>,
}

impl Closet {
    pub fn new(tree: Tree<Item>) -> Closet {
        Closet { tree }
    }

    pub fn outfits(&self) -> BTreeSet<BTreeSet<Item>> {
        self.tree.combinations()
    }
}