#[cfg(test)]
mod tests {
    use zdd::tree::Universe;

    #[test]
    fn restrict_returns_tree_that_has_no_combinations_when_requiring_an_item_that_does_not_exist() {
        let item1 = "1";
        let item2 = "2";

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
        ]);
        let tree = tree.restrict(&[item2.clone()], &[]);

        assert_eq!(
            btreeset!(),
            tree.combinations()
        );
    }

    #[test]
    fn restrict_returns_tree_that_has_no_combinations_when_requiring() {
        let item1 = "1";
        let item2 = "2";

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()],
        ]);
        let tree = tree.restrict(&[item1.clone(), item2.clone()], &[]);

        assert_eq!(
            btreeset!(),
            tree.combinations()
        );
    }

    #[test]
    fn restrict_returns_tree_that_has_one_combination_when_requiring() {
        let item1 = "1";
        let item2 = "2";

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()],
        ]);
        let tree = tree.restrict(&[item1.clone()], &[]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        );
    }

    #[test]
    fn restrict_returns_tree_that_has_many_combination_when_requiring() {
        let item1 = "1";
        let item2 = "2";
        let item3 = "3";

        let universe = Universe::from(vec![item1.clone(), item2.clone(), item3.clone()]);
        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()],
            vec![item3.clone()],
            vec![item1.clone(), item2.clone()],
            vec![item1.clone(), item3.clone()],
            vec![item2.clone(), item3.clone()]
        ]);

        {
            let tree = tree.restrict(&[item1.clone()], &[]);
            assert_eq!(
                btreeset!(
                    btreeset!(item1.clone()),
                    btreeset!(item1.clone(), item2.clone()),
                    btreeset!(item1.clone(), item3.clone())
                ),
                tree.combinations()
            );
        }

        {
            let tree = tree.restrict(&[item1.clone(), item2.clone()], &[]);
            assert_eq!(
                btreeset!(
                    btreeset!(item1.clone(), item2.clone()),
                ),
                tree.combinations()
            );
        }
    }

    #[test]
    fn restrict_returns_tree_that_has_no_combinations_when_excluding() {
        let item1 = "1";
        let item2 = "2";

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()],
        ]);
        let tree = tree.restrict(&[], &[item1.clone(), item2.clone()]);

        assert_eq!(
            btreeset!(),
            tree.combinations()
        );
    }

    #[test]
    fn restrict_returns_tree_that_has_one_combination_when_excluding() {
        let item1 = "1";
        let item2 = "2";

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()],
        ]);
        let tree = tree.restrict(&[], &[item1.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item2.clone())),
            tree.combinations()
        );
    }

    #[test]
    fn restrict_returns_tree_that_has_many_combination_when_excluding() {
        let item1 = "1";
        let item2 = "2";
        let item3 = "3";

        let universe = Universe::from(vec![item1.clone(), item2.clone(), item3.clone()]);
        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()],
            vec![item3.clone()],
            vec![item1.clone(), item2.clone()],
            vec![item1.clone(), item3.clone()],
            vec![item2.clone(), item3.clone()]
        ]);
        let tree = tree.restrict(&[], &[item1.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item2.clone()), btreeset!(item2.clone(), item3.clone()), btreeset!(item3.clone())),
            tree.combinations()
        );
    }
}