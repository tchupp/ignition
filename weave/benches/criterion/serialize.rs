use bincode;
use criterion::Criterion;
use criterion::Fun;
use serde_json;

use weave::zdd::Tree;
use zdd_building;

fn bincode_serialize(tree: &Tree<&str>) {
    bincode::serialize(tree).expect("expected Ok, but was");
}

fn bincode_deserialize(tree: &[u8]) {
    let _: Tree<&str> = bincode::deserialize(tree).expect("expected Ok, but was");
}

fn json_serialize(tree: &Tree<&str>) {
    serde_json::to_string(tree).expect("expected Ok, but was");
}

fn json_deserialize(tree: &[u8]) {
    let _: Tree<&str> = serde_json::from_slice(tree).expect("expected Ok, but was");
}

pub fn bench_serialize(c: &mut Criterion) {
    {
        let bincode_serialize = Fun::new("Bincode", |b, tree| b.iter(|| bincode_serialize(tree)));
        let json_serialize = Fun::new("Json", |b, tree| b.iter(|| json_serialize(tree)));
        let functions = vec!(bincode_serialize, json_serialize);

        let tree = zdd_building::setup_tree_10();
        c.bench_functions("Serialize_10", functions, tree);
    }
    {
        let bincode_serialize = Fun::new("Bincode", |b, tree| b.iter(|| bincode_serialize(tree)));
        let json_serialize = Fun::new("Json", |b, tree| b.iter(|| json_serialize(tree)));
        let functions = vec!(bincode_serialize, json_serialize);

        let tree = zdd_building::setup_tree_20();
        c.bench_functions("Serialize_20", functions, tree);
    }
    {
        let bincode_serialize = Fun::new("Bincode", |b, tree| b.iter(|| bincode_serialize(tree)));
        let json_serialize = Fun::new("Json", |b, tree| b.iter(|| json_serialize(tree)));
        let functions = vec!(bincode_serialize, json_serialize);

        let tree = zdd_building::setup_tree_computer_parts();
        c.bench_functions("Serialize_Computer_Parts", functions, tree);
    }
}

pub fn bench_deserialize(c: &mut Criterion) {
    {
        let bincode_deserialize = Fun::new("Bincode", |b, tree: &Vec<u8>| b.iter(|| bincode_deserialize(tree.as_slice())));
        let functions = vec!(bincode_deserialize);

        let tree = zdd_building::setup_tree_10();
        let tree_string = bincode::serialize(&tree).expect("expected to serialize pre-test, but was");
        c.bench_functions("Deserialize_10", functions, tree_string);
    }
    {
        let json_deserialize = Fun::new("Json", |b, tree: &String| b.iter(|| json_deserialize(tree.as_bytes())));
        let functions = vec!(json_deserialize);

        let tree = zdd_building::setup_tree_10();
        let tree_string = serde_json::to_string(&tree).expect("expected to serialize pre-test, but was");
        c.bench_functions("Deserialize_10", functions, tree_string);
    }

    {
        let bincode_deserialize = Fun::new("Bincode", |b, tree: &Vec<u8>| b.iter(|| bincode_deserialize(tree.as_slice())));
        let functions = vec!(bincode_deserialize);

        let tree = zdd_building::setup_tree_20();
        let tree_string = bincode::serialize(&tree).expect("expected to serialize pre-test, but was");
        c.bench_functions("Deserialize_20", functions, tree_string);
    }
    {
        let json_deserialize = Fun::new("Json", |b, tree: &String| b.iter(|| json_deserialize(tree.as_bytes())));
        let functions = vec!(json_deserialize);

        let tree = zdd_building::setup_tree_20();
        let tree_string = serde_json::to_string(&tree).expect("expected to serialize pre-test, but was");
        c.bench_functions("Deserialize_20", functions, tree_string);
    }

    {
        let bincode_deserialize = Fun::new("Bincode", |b, tree: &Vec<u8>| b.iter(|| bincode_deserialize(tree.as_slice())));
        let functions = vec!(bincode_deserialize);

        let tree = zdd_building::setup_tree_computer_parts();
        let tree_string = bincode::serialize(&tree).expect("expected to serialize pre-test, but was");
        c.bench_functions("Deserialize_Computer_Parts", functions, tree_string);
    }
    {
        let json_deserialize = Fun::new("Json", |b, tree: &String| b.iter(|| json_deserialize(tree.as_bytes())));
        let functions = vec!(json_deserialize);

        let tree = zdd_building::setup_tree_computer_parts();
        let tree_string = serde_json::to_string(&tree).expect("expected to serialize pre-test, but was");
        c.bench_functions("Deserialize_Computer_Parts", functions, tree_string);
    }
}