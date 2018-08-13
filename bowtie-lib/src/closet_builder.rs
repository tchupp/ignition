use core::Family;
use core::Item;

#[derive(Debug, Eq, PartialEq)]
pub enum ClosetBuilderError {
    ConflictingFamilies(Vec<(Item, Vec<Family>)>),
    InclusionError(Vec<(Family, Vec<Item>)>),
    ExclusionError(Vec<(Family, Vec<Item>)>),
}