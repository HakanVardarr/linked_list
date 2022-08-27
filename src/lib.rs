#![allow(unused)]

#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }
    fn push(&mut self, data: T) {
        let old_head = self.head.take();
        let mut new_head = Box::new(Node::new(data));
        new_head.next = old_head;

        self.head = Some(new_head);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = List::new();
        list.push(10);
        list.push(20);
        list.push(30);
    }
}
