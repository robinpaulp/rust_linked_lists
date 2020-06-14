pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None}
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });

        self.head = Some(new_node)
        
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take()
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basiscs() {
        let mut my_list = List::new();

        assert_eq!(my_list.pop(), None);
        assert_eq!(my_list.pop(), None);

        my_list.push(1);
        my_list.push(2);
        my_list.push(3);

        assert_eq!(my_list.pop(), Some(3));
        assert_eq!(my_list.pop(), Some(2));

        my_list.push(4);
        my_list.push(5);

        assert_eq!(my_list.pop(), Some(5));
        assert_eq!(my_list.pop(), Some(4));
        assert_eq!(my_list.pop(), Some(1));
        assert_eq!(my_list.pop(), None);

    }
    #[test]
    fn peek() {
        let mut my_list = List::new();

        assert_eq!(my_list.peek(), None);
        assert_eq!(my_list.peek_mut(), None);

        my_list.push(1);
        my_list.push(2);
        my_list.push(3);

        assert_eq!(my_list.peek(), Some(&3));
        assert_eq!(my_list.peek(), Some(&3));
        assert_eq!(my_list.peek_mut(), Some(&mut 3));
        my_list.peek_mut().map(|val| {
            *val = 42;
        });
    }
}
