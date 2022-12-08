struct Folder {
    file_size: Option<u32>,
    children: Vec<Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}

impl Folder {
    pub fn new() -> Folder {
        return Folder {
            file_size: None,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Folder>>) {
        self.children.push(new_node);
    }

    pub fn print(&self) -> String {
        if let Some(value) = self.value {
            return value.to_string();
        } else {
            return String::from("[")
                + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
                + "]";
        }
    }
}
