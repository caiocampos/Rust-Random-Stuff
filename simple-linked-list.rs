type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    length: usize,
    head: Link<T>,
}

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Link<T>) -> Self {
        Node {
            data: data,
            next: next,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::from(self)
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            length: 0,
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Node::new(element, self.head.take()).boxed());
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            None => None,
            _ => {
                let mut head = self.head.take().unwrap();
                let next = head.next.take();
                self.head = next;
                self.length -= 1;
                Some(head.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            _ => Some(&self.head.as_ref().unwrap().data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let (mut list, mut el) = (Self::new(), &self.head);
        while el.is_some() {
            let node = el.as_ref().unwrap();
            list.push(node.data.clone());
            el = &node.next;
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        item.iter().cloned().fold(Self::new(), |mut list, el| {
            list.push(el);
            list
        })
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(el) = self.pop() {
            vec.push(el);
        }
        vec.into_iter().rev().collect()
    }
}
