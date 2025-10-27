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
    let mut cou_vec: Vec<i32> = vec![1,2,3,4];
    let max_vec: Vec<i32> = vec![1,2,10,3,4];

    println!("Factorial {}", factorial(10));
    println!("Sum {}", sum(&mut sum_vec));
    println!("Count {}", count(&mut cou_vec, 0));
    println!("Max Number {}", max_num(&max_vec, 0, 0));
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

fn count(list: &mut Vec<i32>, c: i32) -> i32 {
    if list.is_empty() {
        return c;
    } else {
        list.pop();
        return count(list, c + 1)
    }
}

fn max_num(list: &Vec<i32>, mut tracker: i32, idx: usize) -> i32 {

    if list.is_empty() {
        return 0;
    }

    if idx > list.len().saturating_sub(1) {
        return tracker
    } else {
        if tracker < list[idx] { tracker = list[idx] };

        return max_num(list, tracker, idx + 1)
    };
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