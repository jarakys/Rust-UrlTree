pub struct Node<T> {
    pub char: char,
    pub value: Option<T>,
    pub children: Option<Vec<Node<T>>>,
}

impl <T> Node<T> {
    fn new(char: char, value: Option<T>) -> Node<T> {
        Node {
            char: char,
            value: value,
            children: None,
        }
    }

    fn add_child(&mut self, char: char) -> &mut Node<T> {
        if self.children.is_some() {
            let children = self.children.as_mut().unwrap();
            //TODO: We need binary search here

            let mut i = 0;
            while i < children.len() && children[i].char < char {
                i += 1;
            }
            if i == children.len() {
                let node = Node::new(char, None);
                children.push(node);
                return children.last_mut().unwrap();
            }

            if children[i].char != char {
                let node = Node::new(char, None);
                children.insert(i, node);
                return &mut children[i];
            }

            &mut children[i]
        } else {
            let node = Node::new(char, None);
            self.children = Some(vec![node]);
            self.children.as_mut().unwrap().last_mut().unwrap()
        }
    }

    pub fn add_rule<F>(&mut self, rule: &str, value_callback: F) where
        F: Fn(Option<&T>) -> T {
        let mut node = self;
        let mut is_wildcard = false;
        for c in rule.chars() {
            let mut should_skip = false;
            if c == '*' {
                is_wildcard = true;
            } else if is_wildcard && c == '?' {
                should_skip = true;
            } else {
                is_wildcard = false;
            }
            if !should_skip {
                node = node.add_child(c);
            }
        }
        node.value = Some(value_callback(node.value.as_ref()));
    }
}