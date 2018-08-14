use bdd::closet::Closet;
use bdd::node::Node;
use core::Item;
use core::Outfit;
use core::OutfitError;


pub fn complete_outfit(closet: &Closet, selections: Vec<Item>) -> Result<Outfit, OutfitError> {
    let mut root: Node = selections.iter()
        .fold(closet.root().clone(), |new_root, selection| Node::restrict(&new_root, selection, true));

    let mut outfit_items = selections;
    loop {
        match root.clone() {
            Node::Branch(id, low, high) => {
                let high = Node::from(high);
                if &high != &Node::FALSE_LEAF {
                    outfit_items.push(id);
                    root = high;
                } else {
                    root = Node::from(low);
                }
            },
            Node::Leaf(_val) => {
                outfit_items.sort();
                return Ok(Outfit::new(outfit_items));
            }
        }
    }
}