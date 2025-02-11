pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: None,
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            data: item,
            next: self.top.take(),
        });
        self.top = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.peek(), Some(&2));
        
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
} 