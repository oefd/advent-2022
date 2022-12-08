use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

const INDENT_SIZE: usize = 2;

/// We wrap the `Node_` so that the consumer of this module doesn't need to
/// worry or think about the ref counting and ref cell use. (Why we need that
/// explained below on the `Node_`)
#[repr(transparent)]
pub struct Node(Rc<RefCell<Node_>>);

impl Node {
    pub fn new_root() -> Self {
        Node(Node_::new_root())
    }

    pub fn name(&self) -> String {
        let inner = self.0.borrow();
        inner.name.clone()
    }

    pub fn parent(&self) -> Option<Node> {
        let inner = self.0.borrow();
        inner.parent.upgrade().map(|strong_ref| Node(strong_ref))
    }

    pub fn children(&self) -> Vec<Node> {
        let inner = self.0.borrow();
        if let Meta::Dir { ref children } = inner.meta {
            children.iter().map(|child| Node(child.clone())).collect()
        } else {
            vec![]
        }
    }

    pub fn is_dir(&self) -> bool {
        let inner = self.0.borrow();
        if let Meta::Dir { .. } = inner.meta {
            true
        } else {
            false
        }
    }

    pub fn add_dir(&mut self, name: String) -> Node {
        self.add_child(Node_ {
            name,
            parent: Rc::downgrade(&self.0),
            meta: Meta::Dir { children: vec![] },
        })
    }

    pub fn add_file(&mut self, name: String, size: usize) -> Node {
        self.add_child(Node_ {
            name,
            parent: Rc::downgrade(&self.0),
            meta: Meta::File { size },
        })
    }

    pub fn size(&self) -> usize {
        let inner = self.0.borrow();
        match &inner.meta {
            Meta::Dir { children } => children.iter().map(|child| child.borrow().size()).sum(),
            Meta::File { size } => *size,
        }
    }

    pub fn print(&self) {
        let inner = self.0.borrow();
        inner.print(0);
    }

    fn add_child(&mut self, child: Node_) -> Node {
        let mut inner = self.0.borrow_mut();
        let child = Rc::new(RefCell::new(child));
        if let Meta::Dir { ref mut children } = inner.meta {
            children.push(child.clone());
            Node(child)
        } else {
            panic!("attempted to add node to non-dir");
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node(self.0.clone())
    }
}

/// A file or dir, which will be wrapped in an Rc<RefCell<_>>.
///
/// # Why an Rc
/// An `Rc<_>` makes the item ref counted, so it'll only drop when all
/// of the many effective owners have dropped their references.
///
/// We need this because we have two owners: The node's parent dir holds a
/// ref to this item, and also the code consuming this module will get a ref.
///
/// Nodes also hold a weak ref to their parent to allow efficient backtracking.
/// The ref must be weak because otherwise a node and its child would form a ref
/// cycle that'd cause a memory leak.
///
/// # Why a RefCell
///
/// The RefCell essentially defers borrow checking to runtime - we need this to
/// allow for mutation even when multiple references to the Node (via the Rc) exist.
struct Node_ {
    name: String,
    parent: Weak<RefCell<Node_>>,
    meta: Meta,
}

enum Meta {
    Dir { children: Vec<Rc<RefCell<Node_>>> },
    File { size: usize },
}

impl Node_ {
    pub fn new_root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: "/".to_string(),
            parent: Weak::new(),
            meta: Meta::Dir { children: vec![] },
        }))
    }

    pub fn size(&self) -> usize {
        match &self.meta {
            Meta::File { size } => *size,
            Meta::Dir { children } => children.iter().map(|child| child.borrow().size()).sum(),
        }
    }

    fn print(&self, indent: usize) {
        match &self.meta {
            Meta::File { size } => {
                println!("{}{} ({})", " ".repeat(indent), self.name, size);
            }
            Meta::Dir { ref children } => {
                let postfix = if self.name == "/" { "" } else { "/" };
                println!("{}{}{}", " ".repeat(indent), self.name, postfix);
                children
                    .iter()
                    .for_each(|child| child.borrow().print(indent + INDENT_SIZE));
            }
        }
    }
}
