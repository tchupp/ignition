use std::cmp::Ordering;
use std::hash::Hash;
use std::iter::FromIterator;

use hashbrown::HashMap;
use itertools::Itertools;

use super::Priority;

mod serialize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Universe<T: Hash + Eq + Clone + Ord> {
    occurrences: HashMap<T, usize>,
    priority: HashMap<T, Priority>,
    index: Vec<T>,
}

impl<T: Hash + Eq + Clone + Ord> Default for Universe<T> {
    fn default() -> Self {
        Universe {
            occurrences: HashMap::new(),
            priority: HashMap::new(),
            index: Vec::new(),
        }
    }
}

impl<T: Hash + Eq + Clone + Ord> Universe<T> {
    pub fn from_items(items: &[T]) -> Self {
        let occurrences: HashMap<T, usize> = items.iter()
            .unique()
            .fold(HashMap::new(), |mut occurrences, item| {
                *occurrences.entry(item.clone()).or_insert(0usize) += 1;
                occurrences
            });

        Universe::from_occurrences(occurrences)
    }

    pub fn from_matrix(matrix: &[Vec<T>]) -> Self {
        let occurrences: HashMap<T, usize> = matrix.iter()
            .map(|set| Self::filter_repeats::<Vec<_>>(set))
            .unique()
            .flatten()
            .fold(HashMap::new(), |mut occurrences, item| {
                *occurrences.entry(item.clone()).or_insert(0usize) += 1;
                occurrences
            });

        Universe::from_occurrences(occurrences)
    }

    fn filter_repeats<B: FromIterator<T>>(set: &[T]) -> B {
        set.iter().cloned().sorted().unique().collect::<B>()
    }

    pub fn merge(&self, other: &Self) -> Self {
        if self == other {
            return self.clone();
        }

        let occurrences: HashMap<T, usize> = other.occurrences.iter()
            .fold(self.occurrences.clone(), |mut occurrences, (item, count)| {
                *occurrences.entry(item.clone()).or_insert(0usize) += count;
                occurrences
            });

        Universe::from_occurrences(occurrences)
    }

    pub fn get_priority(&self, item: &T) -> Option<Priority> {
        self.priority.get(item).cloned()
    }

    pub fn get_priorities<B: FromIterator<Priority>>(&self, items: &[T]) -> B {
        items.iter()
            .filter_map(|item| self.get_priority(item))
            .collect::<B>()
    }

    pub fn get_items<B: FromIterator<T>>(&self, ids: &[Priority]) -> B {
        ids.iter()
            .filter_map(|id| self.get_item(*id))
            .cloned()
            .sorted()
            .collect::<B>()
    }

    pub fn get_item(&self, id: Priority) -> Option<&T> {
        self.index.get(id.0)
    }
}

impl<T: Hash + Eq + Clone + Ord> Universe<T> {
    fn from_occurrences(occurrences: HashMap<T, usize>) -> Self {
        let ordering: fn(&(&T, &usize), &(&T, &usize)) -> Ordering = |(name1, count1), (name2, count2)| Ord::cmp(count2, count1).then(Ord::cmp(name1, name2));

        let priority: HashMap<T, Priority> = occurrences.iter()
            .sorted_by(ordering)
            .map(|(item, _)| item)
            .enumerate()
            .map(|(index, item)| (item.clone(), Priority(index)))
            .collect();

        let index: Vec<T> = occurrences.iter()
            .sorted_by(ordering)
            .map(|(index, _)| index.clone())
            .collect();

        Universe {
            occurrences,
            priority,
            index,
        }
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;

    use super::Priority;
    use super::Universe;

    #[test]
    fn from_matrix() {
        let matrix: &[Vec<&str>] = &[
            vec!["1", "4", "7"],
            vec!["1", "4", "8"],
            vec!["1", "4", "9"],
            vec!["1", "5", "7"],
            vec!["1", "5", "8"],
            vec!["1", "5", "9"],
            vec!["1", "6", "7"],
            vec!["1", "6", "8"],
            vec!["1", "6", "9"],
            vec!["2", "4", "7"],
            vec!["2", "4", "8"],
            vec!["2", "4", "9"],
            vec!["2", "5", "7"],
            vec!["2", "5", "8"],
            vec!["2", "5", "9"],
            vec!["3", "4", "7"],
            vec!["3", "4", "8"],
            vec!["3", "4", "9"],
        ];
        let universe = Universe::from_matrix(matrix);

        let occurrences: HashMap<&str, usize> = hashmap! {
                "1" => 9,
                "4" => 9,
                "2" => 6,
                "5" => 6,
                "7" => 6,
                "8" => 6,
                "9" => 6,
                "6" => 3,
                "3" => 3,
            }.into_iter().collect();
        assert_eq!(occurrences, universe.occurrences);

        let priority: HashMap<&str, Priority> = hashmap! {
                "1" => Priority(0),
                "4" => Priority(1),
                "2" => Priority(2),
                "5" => Priority(3),
                "7" => Priority(4),
                "8" => Priority(5),
                "9" => Priority(6),
                "3" => Priority(7),
                "6" => Priority(8),
            }.into_iter().collect();
        assert_eq!(priority, universe.priority);

        let index: Vec<&str> = vec![
            "1",
            "4",
            "2",
            "5",
            "7",
            "8",
            "9",
            "3",
            "6",
        ];
        assert_eq!(index, universe.index);
    }

    #[test]
    fn from_matrix_does_not_duplicate_occurances() {
        let matrix: &[Vec<&str>] = &[
            vec!["1", "4"],
            vec!["1", "4"],
            vec!["2", "5"],
        ];
        let universe = Universe::from_matrix(matrix);

        let occurrences: HashMap<&str, usize> = hashmap! {
                "1" => 1,
                "4" => 1,
                "2" => 1,
                "5" => 1,
            }.into_iter().collect();
        assert_eq!(occurrences, universe.occurrences);

        let priority: HashMap<&str, Priority> = hashmap! {
                "1" => Priority(0),
                "2" => Priority(1),
                "4" => Priority(2),
                "5" => Priority(3),
            }.into_iter().collect();
        assert_eq!(priority, universe.priority);

        let index: Vec<&str> = vec![
            "1",
            "2",
            "4",
            "5",
        ];
        assert_eq!(index, universe.index);
    }

    #[test]
    fn from_items() {
        let items = vec!["1", "2", "3", "2"];
        let universe = Universe::from_items(&items);

        let occurrences: HashMap<&str, usize> = hashmap! {
                "1" => 1,
                "2" => 1,
                "3" => 1,
            }.into_iter().collect();
        assert_eq!(occurrences, universe.occurrences);

        let priority: HashMap<&str, Priority> = hashmap! {
                "1" => Priority(0),
                "2" => Priority(1),
                "3" => Priority(2),
            }.into_iter().collect();
        assert_eq!(priority, universe.priority);

        let index: Vec<&str> = vec![
            "1",
            "2",
            "3",
        ];
        assert_eq!(index, universe.index);
    }


    #[test]
    fn get_item_is_consistent() {
        let matrix: &[Vec<&str>] = &[
            vec!["1", "4", "7"],
            vec!["1", "4", "8"],
            vec!["1", "5", "8"],
        ];

        let universe = Universe::from_matrix(matrix);

        {
            let priority = universe.get_priority(&"1").unwrap();
            assert_eq!(Priority(0), priority);

            let item = universe.get_item(priority).unwrap();
            assert_eq!(&"1", item);
        }
        {
            let priority = universe.get_priority(&"4").unwrap();
            assert_eq!(Priority(1), priority);

            let item = universe.get_item(priority).unwrap();
            assert_eq!(&"4", item);
        }
        {
            let priority = universe.get_priority(&"7").unwrap();
            assert_eq!(Priority(4), priority);

            let item = universe.get_item(priority).unwrap();
            assert_eq!(&"7", item);
        }
    }

    #[test]
    fn merge() {
        let matrix1: &[Vec<&str>] = &[
            vec!["1", "4", "7"],
        ];
        let universe1 = Universe::from_matrix(matrix1);

        let matrix2: &[Vec<&str>] = &[
            vec!["1", "4", "8"],
            vec!["1", "5", "8"],
        ];
        let universe2 = Universe::from_matrix(matrix2);

        let expected_matrix: &[Vec<&str>] = &[
            vec!["1", "4", "7"],
            vec!["1", "4", "8"],
            vec!["1", "5", "8"],
        ];
        let expected_universe = Universe::from_matrix(expected_matrix);

        assert_eq!(expected_universe, Universe::merge(&universe1, &universe2));
    }
}
