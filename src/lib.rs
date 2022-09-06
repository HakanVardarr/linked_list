type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
    len: usize,
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| &node.data)
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| &mut node.data)
    }
}

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }
    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };

        self.len += 1;
        self.head = Some(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn peak(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    pub fn peak_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current_head = self.head.take();
        while let Some(mut current_head_inner) = current_head.take() {
            let next = current_head_inner.next.take();
            current_head_inner.next = prev.take();
            prev = Some(current_head_inner);
            current_head = next;
        }

        self.head = prev;
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_head = self.head.take();
        while let Some(mut current_head_inner) = current_head {
            current_head = current_head_inner.next.take();
        }
    }
}
