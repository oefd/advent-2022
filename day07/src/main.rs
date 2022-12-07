mod fs_tree;

fn main() {
    let input = util::Stdin::new().cleaned_lines();

    let root = fs_tree::Node::new_root();
    let mut pwd = root.clone();
    for line in input {
        if line.starts_with("$ cd") {
            let dir_to_enter = line.split_ascii_whitespace().skip(2).next().unwrap();
            if dir_to_enter == ".." {
                pwd = pwd.parent().unwrap();
            } else if dir_to_enter == "/" {
                pwd = root.clone();
            } else {
                pwd = pwd
                    .children()
                    .iter()
                    .find(|child| child.name() == dir_to_enter)
                    .unwrap()
                    .clone();
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            let (meta, name) = line.split_once(' ').unwrap();
            if meta == "dir" {
                pwd.add_dir(name.to_string());
            } else {
                let size: usize = meta.parse().unwrap();
                pwd.add_file(name.to_string(), size);
            }
        }
    }

    root.print();
    println!("size sum 100_000: {}", find_size_sum(&root, 100_000));

    let dir_to_del = find_to_delete(&root);
    println!(
        "dir {} with size {} should be deleted",
        dir_to_del.name(),
        dir_to_del.size()
    );
}

fn find_size_sum(root: &fs_tree::Node, max: usize) -> usize {
    assert!(root.is_dir());
    let mut sum = 0;

    if root.size() <= max {
        sum += root.size();
    }

    for child in root.children() {
        if child.is_dir() {
            sum += find_size_sum(&child, max);
        }
    }
    sum
}

fn find_to_delete(root: &fs_tree::Node) -> fs_tree::Node {
    assert!(root.is_dir());

    const DISK_SIZE: usize = 70_000_000;
    const SIZE_NEEDED: usize = 30_000_000;

    let unused_space = DISK_SIZE - root.size();
    let must_delete = SIZE_NEEDED - unused_space;

    find_to_delete_(root, must_delete).unwrap()
}

fn find_to_delete_(root: &fs_tree::Node, min_size: usize) -> Option<fs_tree::Node> {
    let mut smallest: Option<fs_tree::Node> = None;
    if root.size() >= min_size {
        smallest = Some(root.clone());
    }
    for child in root.children() {
        if child.is_dir() {
            let smallest_child = find_to_delete_(&child, min_size);
            if let Some(smallest_child) = smallest_child {
                if smallest_child.size() < smallest.as_ref().map(|s| s.size()).unwrap_or(usize::MAX)
                {
                    smallest = Some(smallest_child)
                }
            }
        }
    }

    smallest
}
