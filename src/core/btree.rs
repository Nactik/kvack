// Struct
struct Node {
    next: Option<Box<Node>>,
    value: String,
    key: i32,
}

pub struct Btree {
    root: Option<Box<Node>>,
    size: usize,
    balance: i32,
}

// Methods

impl Node {
    fn new(value: i32) -> Node {
        Node {
            left: None,
            right: None,
            next: None,
            value,
        }
    }

    fn is_leaf(&self) -> bool {
        return !self.value.is_empty();
    }
}

impl Btree {
    pub fn get(&self, key: &u32) -> Option<u32> {}

    pub fn insert(&mut self, key: &u32, value: String) -> Result<String, String> {
        Ok("".to_string())
    }

    pub fn delete(&mut self, key: &u32) -> Result<u32, String> {
        Ok("".to_string())
    }

    pub fn update(&mut self, key: &u32, value: String) -> Result<String, String> {
        Ok("".to_string())
    }

    pub fn new() -> Self {
        Btree {
            root: Node::new(0),
            size: 0,
            balance: 4, // balance = 1 + (size of ram / size of key type )
        }
    }
}
