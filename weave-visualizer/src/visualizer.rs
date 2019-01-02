use std::collections::BTreeSet;

use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    Renderable,
};

use weave::{
    Tree,
    TreeNode,
    Universe,
};

pub struct Model {
    tree: Tree<String>
}

pub enum Msg {
    Nope
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let shirt_red = String::from("red shirt");
        let shirt_blue = String::from("blue shirt");

        let jeans_black = String::from("black jeans");
        let jeans_blue = String::from("blue jeans");
        let jeans_purple = String::from("purple jeans");

        let blue_shoes = String::from("blue shoes");
        let flip_flops = String::from("flip flops");

        let universe = Universe::from(vec![
            shirt_red.clone(),
            shirt_blue.clone(),
            jeans_black.clone(),
            jeans_blue.clone(),
            jeans_purple.clone(),
            blue_shoes.clone(),
            flip_flops.clone(),
        ]);
        let tree = universe.hyper_tree(&[
            vec![shirt_red.clone(), jeans_black.clone(), blue_shoes.clone()],
            vec![shirt_blue.clone(), jeans_black.clone(), blue_shoes.clone()],
            vec![shirt_red.clone(), jeans_blue.clone(), blue_shoes.clone()],
            vec![shirt_blue.clone(), jeans_blue.clone(), blue_shoes.clone()],
            vec![shirt_red.clone(), jeans_purple.clone(), blue_shoes.clone()],
            vec![shirt_blue.clone(), jeans_purple.clone(), blue_shoes.clone()],
            vec![shirt_red.clone(), jeans_black.clone(), flip_flops.clone()],
            vec![shirt_blue.clone(), jeans_black.clone(), flip_flops.clone()],
            vec![shirt_red.clone(), jeans_blue.clone(), flip_flops.clone()],
            vec![shirt_blue.clone(), jeans_blue.clone(), flip_flops.clone()],
            vec![shirt_red.clone(), jeans_purple.clone(), flip_flops.clone()],
            vec![shirt_blue.clone(), jeans_purple.clone(), flip_flops.clone()],
        ]);

        Model { tree }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Nope => {}
        }

        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Model> {
        html! {
            <div class="weave-viz-wrapper",>
                <section class="weaveviz-app",>
                    <header class="header",>
                        <h1>{ "Weave Viz" }</h1>
                    </header>
                    <section class="main",>
                        <div class="combo-wrapper",>
                            { self.view_combinations() }
                        </div>
                        <div class="tree-wrapper",>
                            { self.view_tree() }
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}

impl Model {
    fn view_tree(&self) -> Html<Self> {
        Model::view_node(0, self.tree.traverse())
    }

    fn view_node(depth: usize, node: TreeNode<String>) -> Html<Self> {
        match node {
            TreeNode::Branch { value, low, high } => Model::view_branch(depth + 1, value, *low, *high),
            TreeNode::Leaf { value } => Model::view_leaf(depth + 1, value)
        }
    }

    fn view_branch(depth: usize, value: String, low: TreeNode<String>, high: TreeNode<String>) -> Html<Self> {
        html! {
            <div class="tree-branch",>
                <label class="tree-branch-label",>{ value }</label>
                <div class="tree-branch-children",>
                { Model::view_node(depth, low) }
                { Model::view_node(depth, high) }
                </div>
            </div>
        }
    }

    fn view_leaf(depth: usize, value: bool) -> Html<Self> {
        html! {
            <div class="tree-leaf",>
                <label>{ value }</label>
            </div>
        }
    }
}

impl Model {
    fn view_combinations(&self) -> Html<Self> {
        let render_combo: fn(BTreeSet<String>) -> Html<Self> = |combo| html! {
            <li>
                <div class="view",>
                    <label>
                        { format!("{}", combo.into_iter().collect::<Vec<String>>().join(", ")) }
                    </label>
                </div>
            </li>
        };

        html! {
            <ul class="combo-list",>
                { for self.tree.combinations().into_iter().map(render_combo) }
            </ul>
        }
    }
}