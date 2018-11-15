use std::collections::BTreeSet;

use core::Item;
use weave::Tree;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

    pub fn outfits_with(&self, selections: &[Item], exclusions: &[Item]) -> BTreeSet<BTreeSet<Item>> {
        self.tree.combinations_with(selections, exclusions)
    }
}