struct Stack {
    items: Vec<i32>,
    size: usize,
}

fn new(size: usize) -> Stack {
    Stack { items: Vec::new(), size }
}

fn isEmpty(stack: &Stack) -> bool {
    stack.items.is_empty()
}

fn isFull(stack: &Stack) -> bool {
    stack.items.len() == stack.size
}

fn peek(stack: &Stack) -> Option<i32> {
    stack.items.last().cloned()
}

fn push(stack: &mut Stack, item: i32) {
    stack.items.push(item);
}

fn pop(stack: &mut Stack) -> Option<i32> {
    stack.items.pop()
}

fn main() {
    let mut stack = new(5);

    push(&mut stack, 1);
    push(&mut stack, 2);
    push(&mut stack, 3);

    println!("Stack is empty: {}", isEmpty(&stack));
    println!("Stack is full: {}", isFull(&stack));
    println!("Top item: {:?}", peek(&stack));

    while !isEmpty(&stack) {
        if let Some(item) = pop(&mut stack) {
            println!("Popped item: {}", item);
        }
    }

    println!("Stack is empty: {}", isEmpty(&stack));
}