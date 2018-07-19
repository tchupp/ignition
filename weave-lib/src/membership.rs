use membership::OutfitError::MultipleItemsPerFamily;
use membership::OutfitError::UnknownItems;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq)]
enum OutfitError {
    UnknownItems(Vec<Item>),
    MultipleItemsPerFamily(BTreeMap<Family, Vec<Item>>),
}

#[derive(Debug, Clone, PartialEq)]
struct Closet<'a> {
    contents: BTreeMap<&'a Family, Vec<&'a Item>>,
    item_index: BTreeMap<&'a Item, &'a Family>,
}

impl<'a> Closet<'a> {
    fn new() -> Closet<'a> {
        Closet {
            contents: BTreeMap::new(),
            item_index: BTreeMap::new(),
        }
    }

    fn add_item(&self, family: &'a Family, item: &'a Item) -> Closet {
        let mut contents = self.contents.clone();
        contents.entry(family)
            .or_insert(vec![])
            .push(item);

        let mut item_index = self.item_index.clone();
        item_index.entry(item)
            .or_insert(family);

        Closet { contents, item_index }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Family(String);

impl Family {
    fn new(id: &str) -> Family {
        Family(id.into())
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Item(String);

impl Item {
    fn new(id: &str) -> Item {
        Item(id.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Outfit<'a> {
    items: Vec<&'a Item>
}

impl<'a> Outfit<'a> {
    fn new(items: Vec<&'a Item>) -> Outfit {
        Outfit { items }
    }
}

fn find_first_valid_outfit<'a>(closet: Closet<'a>, selections: Vec<&'a Item>) -> Result<Outfit<'a>, OutfitError> {
    if let Some(items) = find_unknown_items(&closet, &selections) {
        return Err(UnknownItems(items));
    }
    if let Some(items) = find_duplicate_items(&closet, &selections) {
        return Err(MultipleItemsPerFamily(items));
    }

    let selected_families: Vec<&Family> = selections.iter()
        .map(|item| closet.item_index.get(item))
        .filter(|family| family.is_some())
        .map(|family| family.unwrap())
        .cloned()
        .collect();

    let items = closet.contents.iter()
        .filter(|&(family, _)| !selected_families.contains(family))
        .map(|(_, items)| items.first())
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .chain(&selections)
        .cloned()
        .collect::<Vec<&Item>>();

    return Ok(Outfit::new(items));
}

fn find_unknown_items(closet: &Closet, selections: &Vec<&Item>) -> Option<Vec<Item>> {
    let unknown_items = selections.iter()
        .filter(|&&item| !(closet.item_index.contains_key(&item)))
        .cloned()
        .cloned()
        .collect::<Vec<Item>>();

    if unknown_items.is_empty() {
        None
    } else {
        Some(unknown_items)
    }
}

fn find_duplicate_items(closet: &Closet, selections: &Vec<&Item>) -> Option<BTreeMap<Family, Vec<Item>>> {
    let duplicates: BTreeMap<Family, Vec<Item>> = selections.iter()
        .map(|item| (closet.item_index.get(item), item))
        .map(|(family, item)| (family.unwrap().clone(), item.clone()))
        .fold(BTreeMap::new(), |mut duplicates, (family, item)| {
            duplicates.entry(family.clone()).or_insert(vec![]).push(item.clone());
            duplicates
        })
        .iter()
        .filter(|&(_, items)| items.len() > 1)
        .map(|(family, items)| (family.clone(), items.into_iter().cloned().collect()))
        .collect()
    ;

    if duplicates.is_empty() {
        None
    } else {
        Some(duplicates)
    }
}

#[cfg(test)]
mod tests {
    use membership::OutfitError::MultipleItemsPerFamily;
    use membership::OutfitError::UnknownItems;
    use super::*;

    #[test]
    fn no_rules_no_selections() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet = Closet::new();
        let closet = closet.add_item(&shirts, &blue);
        let closet = closet.add_item(&shirts, &red);
        let closet = closet.add_item(&pants, &jeans);
        let closet = closet.add_item(&pants, &slacks);

        assert_eq!(
            Ok(Outfit::new(vec![&jeans, &blue])),
            find_first_valid_outfit(closet, vec![])
        );
    }

    #[test]
    fn no_rules_one_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet = Closet::new();
        let closet = closet.add_item(&shirts, &blue);
        let closet = closet.add_item(&shirts, &red);
        let closet = closet.add_item(&pants, &jeans);
        let closet = closet.add_item(&pants, &slacks);

        assert_eq!(
            Ok(Outfit::new(vec![&jeans, &red])),
            find_first_valid_outfit(closet, vec![&red])
        );
    }

    #[test]
    fn no_rules_selection_for_each_family() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet = Closet::new();
        let closet = closet.add_item(&shirts, &blue);
        let closet = closet.add_item(&shirts, &red);
        let closet = closet.add_item(&pants, &jeans);
        let closet = closet.add_item(&pants, &slacks);

        assert_eq!(
            Ok(Outfit::new(vec![&slacks, &blue])),
            find_first_valid_outfit(closet, vec![&slacks, &blue])
        );
    }

    #[test]
    fn no_rules_unknown_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");
        let black = Item::new("black");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet = Closet::new();
        let closet = closet.add_item(&shirts, &blue);
        let closet = closet.add_item(&shirts, &red);
        let closet = closet.add_item(&pants, &jeans);
        let closet = closet.add_item(&pants, &slacks);

        assert_eq!(
            Err(UnknownItems(vec![black.clone()])),
            find_first_valid_outfit(closet, vec![&jeans, &black])
        );
    }

    #[test]
    fn no_rules_more_selections_than_families() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet = Closet::new();
        let closet = closet.add_item(&shirts, &blue);
        let closet = closet.add_item(&shirts, &red);
        let closet = closet.add_item(&pants, &jeans);
        let closet = closet.add_item(&pants, &slacks);

        let mut duplicates = BTreeMap::new();
        duplicates.insert(pants.clone(), vec![jeans.clone(), slacks.clone()]);

        assert_eq!(
            Err(MultipleItemsPerFamily(duplicates)),
            find_first_valid_outfit(closet, vec![&jeans, &blue, &slacks])
        );
    }
}