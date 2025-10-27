use std::{ops::Add, usize};

trait Removable {
    type Item;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn remove(&mut self, index: usize) -> Self::Item;
}

impl Removable for Vec<i32> {
    type Item = i32;

    fn remove(&mut self, index: usize) -> Self::Item {Vec::remove(self, index)}
    fn is_empty(&self) -> bool {Vec::is_empty(&self)}
    fn len(&self) -> usize {Vec::len(&self)} 
}

fn main() {
    let mut sum_vec: Vec<i32> = vec![1,2,3,4];

    println!("{}", factorial(10));
    println!("{}", sum(&mut sum_vec));
}

fn factorial(n: i32) -> i32 {
    if n <= 1 {return n; }
    else { return n * factorial(n - 1) };
}

// It's possible to copy the array and mut the copy instead of original array
fn sum<T>(n: &mut T) -> i32 where 
    T: Removable,
    <T as Removable>::Item: Add<i32, Output = i32>
{
    let len: usize = n.len().saturating_sub(1);

    if n.is_empty() { 
        return 0 
    } else { 
        return n.remove(len) + sum(n) 
    }
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

// ADDITIONAL NOTES:

/*
    A function in the call stack is in a partial complete state. It's: the function
    does not finished yet. 

    When the top function in the stack is out, the following start again from where
    it stopped! 
*/

// NOTES FOR OTHER READERS:

/*
    I'm so sorry by the bad English. I'm still learning it. 
*/