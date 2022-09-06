#![allow(unused)]

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
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
        let mut new_node = Node::new(data);
        let cur_head = self.head.take();
        new_node.next = cur_head;
        self.head = Some(Box::new(new_node));
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
    fn peak(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    fn peak_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_head = self.head.take();
        while let Some(mut node) = cur_head {
            cur_head = node.next.take();
        }
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

        list.pop();

        assert_eq!(Some(&20), list.peak());
        list.peak_mut().map(|value| *value = 12);

        println!("{:?}", list);
    }
}
