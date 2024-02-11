// https://leetcode.com/problems/implement-queue-using-stacks

struct MyQueue {
    enqueue: Vec<i32>,
    dequeue: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            enqueue: Vec::new(),
            dequeue: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.enqueue.push(x)
    }

    fn pop(&mut self) -> i32 {
        self.rotate();
        self.dequeue.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.rotate();
        *self.dequeue.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.enqueue.is_empty() && self.dequeue.is_empty()
    }

    fn rotate(&mut self) {
        if self.dequeue.is_empty() {
            while !self.enqueue.is_empty() {
                self.dequeue.push(self.enqueue.pop().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::queue_using_stacks::MyQueue;

    #[test]
    fn test_myqueue_case1() {
        let mut my_queue: MyQueue = MyQueue::new();
        my_queue.push(1);
        my_queue.push(2);
        assert_eq!(my_queue.peek(), 1);
        assert_eq!(my_queue.pop(), 1);
        assert_eq!(my_queue.empty(), false);
        assert_eq!(my_queue.peek(), 2);
        assert_eq!(my_queue.pop(), 2);
        assert_eq!(my_queue.empty(), true);
    }
}
