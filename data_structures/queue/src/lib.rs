pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        let mut new_node = Box::new(Node {
            data: item,
            next: None,
        });
        
        let new_node_ptr = &mut *new_node as *mut Node<T>;
        
        match self.tail {
            Some(tail_ptr) => unsafe {
                (*tail_ptr).next = Some(new_node);
            },
            None => {
                self.head = Some(new_node);
            }
        }
        
        self.tail = Some(new_node_ptr);
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            
            if self.head.is_none() {
                self.tail = None;
            }
            
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
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
    fn test_queue_operations() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.peek(), Some(&1));
        
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }
} 