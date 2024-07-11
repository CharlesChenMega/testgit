use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
struct TreeNode<T> where T: Eq + Hash + Clone {
    value: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> where T: Eq + Hash + Clone {
    fn new(value: T) -> Self {
        TreeNode { value, children: vec![] }
    }
}

fn build_tree<T>(tuples: Vec<(T, T)>) -> Vec<TreeNode<T>>
where
    T: Eq + Hash + Clone + Debug,
{
    let mut map: HashMap<T, Vec<T>> = HashMap::new();
    let mut is_child: HashSet<T> = HashSet::new();

    for (parent, child) in tuples {
        map.entry(parent).or_default().push(child.clone());
        is_child.insert(child);
    }

    let roots: Vec<T> = map.keys().cloned().filter(|k| !is_child.contains(k)).collect();

    fn build_node<T>(value: T, map: &HashMap<T, Vec<T>>) -> TreeNode<T>
    where
        T: Eq + Hash + Clone + Debug,
    {
        let mut node = TreeNode::new(value.clone());
        if let Some(children) = map.get(&value) {
            for child in children {
                node.children.push(build_node(child.clone(), map));
            }
        }
        node
    }

    roots.into_iter().map(|root| build_node(root, &map)).collect()
}
fn find_leaves<T>(node: &TreeNode<T>) -> Vec<T>
where
    T: Eq + Hash + Clone + Debug,
{
    if node.children.is_empty() {
        return vec![node.value.clone()];
    }
    let mut leaves = Vec::new();
    for child in &node.children {
        leaves.extend(find_leaves(child));
    }
    leaves
}
fn main() {
    let tuples = vec![('a', 'b'), ('b', 'c'), ('b','h'),('c', 'd'), ('c', 'e'), ('e','m'),('m','t'),('f', 'g')];
    let trees = build_tree(tuples);

    for tree in &trees {
        println!("Root: {:?}", tree.value);
        // Leaves printing logic can be added here
        let leaves = find_leaves(tree);
        println!("Leaves: {:?}", leaves);
    }
}


