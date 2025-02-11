use stack::Stack;
use queue::Queue;

fn main() {
    test_stack();
    test_queue();
}

fn test_stack() {
    println!("=== 스택 테스트 ===");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("스택 크기: {}", stack.size());
    println!("맨 위 원소: {:?}", stack.peek());
    println!("pop: {:?}", stack.pop());
    println!("새로운 크기: {}", stack.size());
}

fn test_queue() {
    println!("\n=== 큐 테스트 ===");
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    
    println!("큐 크기: {}", queue.size());
    println!("첫 번째 원소: {:?}", queue.peek());
    println!("dequeue: {:?}", queue.dequeue());
    println!("새로운 크기: {}", queue.size());
} 