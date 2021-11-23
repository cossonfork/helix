/*
tree所做的事情：运用tree算法将 普通list 转换成 树型list
*/

use std::path::{Component, Components, Path, PathBuf};
use std::{cell::Cell, collections::HashMap, rc::Rc};

trait IntoTreeItem {
    fn into_tree_path();
}

struct Node {
    text: String,
    childs: Vec<Rc<Cell<String>>>,
}

struct FileTree<T> {
    origin: Vec<PathBuf>,
    tree_list: Vec<(PathBuf, usize)>,
}

struct HT(HashMap<PathBuf, Self>);

impl<T> Tree<T> {
    fn make_tree(paths: Vec<PathBuf>) {
        let mut tree = HT(HashMap::new());
        for path in paths {
            let childs = &mut tree;
            for com in path.components() {
                let childs = childs.entry(com).or_insert_with(|| HT(HashMap::new()));
            }
        }

        fn walk(
            parent_componts: &mut Vec<Component>,
            tree: &HashMap<Component, Self>,
            collector: &mut Vec<PathBuf>,
        ) {
            for (key, childs) in tree {
                parent_componts.push(key);
                let coms = Components::from(&parent_componts);
                collector.push(coms.as_path().to_path_buf());
            }
        }
    }
}

fn into_tree_style_cell(list: Vec<String>) {
    println!("")
}
