use hashbrown::HashMap;

use weave::Forest;

struct FeatureId(usize);

type Feature<'a> = &'a str;
type Features<'a> = Vec<Feature<'a>>;

type Family<'a> = &'a str;
type Families<'feat, 'fam> = HashMap<Feature<'feat>, Family<'fam>>;

type Configurations<'a> = Forest<Feature<'a>>;

pub struct Catalog<'a> {
    features: Features<'a>,
    families: Families<'a, 'a>,
}
