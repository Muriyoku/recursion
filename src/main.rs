fn main() {
    println!("{}", factorial(10));
}

fn factorial(n: i32) -> i32 {
    if n <= 1 {return n; }
    else { return n * factorial(n - 1) };
}

// SEARCH: tail recursion

// call stack is a data structure used by the OS to save function call 
// each new function call add a information about the new function in the
// top of stack. Information include variables, parameters and so on...

// In other words, you put something at the top of stack, remove it 
// first. The base is the last of stack goes out. It's called of 
// LIFO (Last in, First out)

// IMPORTANT: A stack is a simple data structure with a push and pop method. 
// it's with a queue data structure, but in a queue, the last out, the first in.