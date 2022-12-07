mod fs_tree;

fn main() {
    let input = util::Stdin::new().cleaned_lines();
    let root = fs_tree::Node::new_root();
}
