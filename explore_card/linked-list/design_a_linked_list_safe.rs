struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    len: u32,
}

/**
*  `&self` means the method takes an immutable reference.
*  If you need a mutable reference, change it to `&mut self` instead.
*/
impl MyLinkedList {
    fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.len as i32 {
            return -1;
        }
        let mut temp = self.head.as_ref().unwrap();

        for _ in 0..index {
            temp = temp.next.as_ref().unwrap();
        }

        return temp.val;
    }

    fn add_at_head(&mut self, val: i32) {
        let temp = self.head.take();
        self.head = Some(Box::new(Node {
            val: val,
            next: temp,
        }));
        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_tail = Some(Box::new(Node {
            val: val,
            next: None,
        }));

        if self.head.is_none() {
            self.head = new_tail;
        } else {
            let mut temp = self.head.as_mut().unwrap();
            while temp.next.is_some() {
                temp = temp.next.as_mut().unwrap();
            }
            temp.next = new_tail;
        }

        self.len += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        match index {
            0 => self.add_at_head(val),
            index if index > self.len as i32 => (),
            index if index == self.len as i32 => self.add_at_tail(val),
            _ => {
                let mut temp = self.head.as_mut().unwrap();
                for _ in 0..index - 1 {
                    temp = temp.next.as_mut().unwrap();
                }
                let next_node = temp.next.take().unwrap();
                temp.next = Some(Box::new(Node {
                    val: val,
                    next: Some(next_node),
                }));

                self.len += 1;
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index >= self.len as i32 {
            return;
        }

        match (self.len, index) {
            (1, 0) => self.head = None,
            (_, 0) => self.head = self.head.as_mut().unwrap().next.take(),
            _ => {
                let mut temp = self.head.as_mut().unwrap();
                for _ in 0..index - 1 {
                    temp = temp.next.as_mut().unwrap();
                }
                let mut delete_node = temp.next.take().unwrap();
                let next_node = delete_node.next.take();
                temp.next = next_node;
            }
        }

        self.len += 1;
    }
}
