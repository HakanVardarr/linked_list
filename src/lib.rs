#![allow(unused)]

#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

struct IntoIter<T>(List<T>);
struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
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
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
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
    fn reverse(&mut self) {
        let mut prev = None;
        let mut current_node = self.head.take();

        while let Some(mut current_node_inner) = current_node.take() {
            let next = current_node_inner.next.take();
            current_node_inner.next = prev.take();
            prev = Some(current_node_inner);
            current_node = next;
        }

        self.head = prev.take();
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
