use bdd::closet::Closet;
use bdd::node;
use bdd::node::Node;
use std::cmp::Ordering;

impl Closet {
    pub fn node_count(&self) -> u64 {
        Closet::node_count_internal(self.root())
    }

    fn node_count_internal(node: &Node) -> u64 {
        match node {
            Node::Leaf(_val) => 1,
            Node::Branch(_id, low, high) => {
                let low = node::get(*low);
                let high = node::get(*high);

                1 + Closet::node_count_internal(&low) + Closet::node_count_internal(&high)
            }
        }
    }

    pub fn leaf_count(&self) -> u64 {
        Closet::leaf_count_internal(self.root())
    }

    fn leaf_count_internal(node: &Node) -> u64 {
        match node {
            Node::Leaf(_val) => 1,
            Node::Branch(_id, low, high) => {
                let low = node::get(*low);
                let high = node::get(*high);

                Closet::leaf_count_internal(&low) + Closet::leaf_count_internal(&high)
            }
        }
    }

    pub fn outfit_count(&self) -> u64 {
        Closet::outfit_count_internal(self.root())
    }

    fn outfit_count_internal(node: &Node) -> u64 {
        match node {
            Node::Leaf(val) => if *val { 1 } else { 0 },
            Node::Branch(_id, low, high) => {
                let low = node::get(*low);
                let high = node::get(*high);

                Closet::outfit_count_internal(&low) + Closet::outfit_count_internal(&high)
            }
        }
    }

    pub fn depth(&self) -> u64 {
        Closet::depth_internal(self.root())
    }

    fn depth_internal(node: &Node) -> u64 {
        match node {
            Node::Leaf(_val) => 1,
            Node::Branch(_id, low, high) => {
                let low = node::get(*low);
                let high = node::get(*high);
                let low_depth = Closet::depth_internal(&low);
                let high_depth = Closet::depth_internal(&high);

                match low_depth.cmp(&high_depth) {
                    Ordering::Less => 1 + high_depth,
                    Ordering::Equal => 1 + low_depth,
                    Ordering::Greater => 1 + low_depth,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bdd::closet_builder::ClosetBuilder;
    use core::Family;
    use core::Item;

    #[test]
    fn count_nodes_families_2_items_4() {
        let shirt1 = Item::new("shirts:1");
        let shirt2 = Item::new("shirts:2");

        let pants1 = Item::new("pants:1");
        let pants2 = Item::new("pants:2");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &shirt1)
            .add_item(&shirts, &shirt2)
            .add_item(&pants, &pants1)
            .add_item(&pants, &pants2);

        let closet = closet_builder.must_build();

        assert_eq!(19, closet.node_count());
        assert_eq!(10, closet.leaf_count());
        assert_eq!(4, closet.outfit_count());
        assert_eq!(5, closet.depth());
    }

    #[test]
    fn count_nodes_families_2_items_4_one_exclusion() {
        let shirt1 = Item::new("shirts:1");
        let shirt2 = Item::new("shirts:2");

        let pants1 = Item::new("pants:1");
        let pants2 = Item::new("pants:2");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &shirt1)
            .add_item(&shirts, &shirt2)
            .add_item(&pants, &pants1)
            .add_item(&pants, &pants2)
            .add_exclusion_rule(&shirt1, &pants1);

        let closet = closet_builder.must_build();

        assert_eq!(17, closet.node_count());
        assert_eq!(9, closet.leaf_count());
        assert_eq!(3, closet.outfit_count());
        assert_eq!(5, closet.depth());
    }

    #[test]
    fn count_nodes_families_2_items_16() {
        let shirt1 = Item::new("shirts:1");
        let shirt2 = Item::new("shirts:2");
        let shirt3 = Item::new("shirts:3");
        let shirt4 = Item::new("shirts:4");
        let shirt5 = Item::new("shirts:5");
        let shirt6 = Item::new("shirts:6");
        let shirt7 = Item::new("shirts:7");
        let shirt8 = Item::new("shirts:8");

        let pants1 = Item::new("pants:1");
        let pants2 = Item::new("pants:2");
        let pants3 = Item::new("pants:3");
        let pants4 = Item::new("pants:4");
        let pants5 = Item::new("pants:5");
        let pants6 = Item::new("pants:6");
        let pants7 = Item::new("pants:7");
        let pants8 = Item::new("pants:8");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &shirt1)
            .add_item(&shirts, &shirt2)
            .add_item(&shirts, &shirt3)
            .add_item(&shirts, &shirt4)
            .add_item(&shirts, &shirt5)
            .add_item(&shirts, &shirt6)
            .add_item(&shirts, &shirt7)
            .add_item(&shirts, &shirt8)
            .add_item(&pants, &pants1)
            .add_item(&pants, &pants2)
            .add_item(&pants, &pants3)
            .add_item(&pants, &pants4)
            .add_item(&pants, &pants5)
            .add_item(&pants, &pants6)
            .add_item(&pants, &pants7)
            .add_item(&pants, &pants8);

        let closet = closet_builder.must_build();

        assert_eq!(649, closet.node_count());
        assert_eq!(325, closet.leaf_count());
        assert_eq!(64, closet.outfit_count());
        assert_eq!(17, closet.depth());
    }

    #[test]
    fn count_nodes_families_4_items_16() {
        let shirt1 = Item::new("shirts:1");
        let shirt2 = Item::new("shirts:2");
        let shirt3 = Item::new("shirts:3");
        let shirt4 = Item::new("shirts:4");

        let pants1 = Item::new("pants:1");
        let pants2 = Item::new("pants:2");
        let pants3 = Item::new("pants:3");
        let pants4 = Item::new("pants:4");

        let shoes1 = Item::new("shoes:1");
        let shoes2 = Item::new("shoes:2");
        let shoes3 = Item::new("shoes:3");
        let shoes4 = Item::new("shoes:4");

        let socks1 = Item::new("socks:1");
        let socks2 = Item::new("socks:2");
        let socks3 = Item::new("socks:3");
        let socks4 = Item::new("socks:4");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");
        let socks = Family::new("socks");
        let shoes = Family::new("shoes");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &shirt1)
            .add_item(&shirts, &shirt2)
            .add_item(&shirts, &shirt3)
            .add_item(&shirts, &shirt4)
            .add_item(&pants, &pants1)
            .add_item(&pants, &pants2)
            .add_item(&pants, &pants3)
            .add_item(&pants, &pants4)
            .add_item(&socks, &socks1)
            .add_item(&socks, &socks2)
            .add_item(&socks, &socks3)
            .add_item(&socks, &socks4)
            .add_item(&shoes, &shoes1)
            .add_item(&shoes, &shoes2)
            .add_item(&shoes, &shoes3)
            .add_item(&shoes, &shoes4);

        let closet = closet_builder.must_build();

        assert_eq!(1701, closet.node_count());
        assert_eq!(851, closet.leaf_count());
        assert_eq!(256, closet.outfit_count());
        assert_eq!(17, closet.depth());
    }
}