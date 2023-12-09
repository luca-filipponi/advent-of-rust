#![allow(dead_code)]
#![warn(unused_variables)]


fn ownership_playground() {


    //this is a string literal and is immutable
    // size is known at compilte time and allocated to the stack
    let _a = "soemthing";

    //new scope;
    {
        let _s = "something else";
    }

    // print!(s); //s can't be found here

    // this is allocated to the stack beacuse this is a mutable string that can change
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // when s will go out of scope, will be deallocate, so far nothing too different
    // compared to a garbage collected language (a paart that the garbage collector will clear)
    // in a gc cycle and rust will clear when goes out of scope

    // this is allowed
    let x = 5;
    let y = x;

    println!("{} {}", x,y); // this is fine

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); //this don't compile, s1 can' t be used anymore
    // error[E0382]: borrow of moved value: `s1`
    // --> src/ownership.rs:36:28
    //      |
    //   33 |     let s1 = String::from("hello");
    //      |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    //   34 |     let s2 = s1;
    //      |              -- value moved here
    //   35 |
    //   36 |     println!("{}, world!", s1); //this don't compile, s1 can' t be used anymore
    // |                            ^^ value borrowed here after move

    // we need to clone it:
    let s3 = s2.clone();

    println!("{}, world!", s2); //this don't compile, s1 can' t be used anymore

    // passing a variable that points to the heap to a function have the same effect
    // of assigning that to another variable is a move.

    takes_ownership(s3);

    // println!("{}, world!", s3); //this don't compile, the method has taken ownership of s3,
    // and when that will return s3 is gone

    let x = 5;
    makes_copy(x);
    println!("{}", x); // this is fine, x is a heap variable is copied over is immutable


    let a1 = String::from("something");
    let a2 = takes_and_gives_back(a1);

    // println!("{}, world!", a1); a1 is a goner, but can use a3

    println!("{}", a2); //this is fine

    let s_lit = "something";
    using_string_literal(s_lit);

    println!("{}", s_lit); //this is fine as well, because is not a mutable string

    let slen = String::from("something");
    let (slen2, len) = calculate_length(slen); //this compile but a bit annoying

    // println!("{}", slen); // this don't compile

    let _slen3 = calculate_length_with_borrowing(&slen2); //this compile but a bit annoying

    println!("{}", slen2); // this works


}

async fn test() {
    let mut data = 42;

    {
        let reference = &mut data; // Mutable borrow starts
        async_function(reference).await;
    } // Mutable borrow ends

    println!("Updated value: {}", data);
}

// what if i need to calculate the size of a string
// the string will be unusable before, one mitigation is to return the string again all the time

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
// by using a refence, this function don't get ownership, so the original
// variable is still valid, the price to pay is that you can't modify it
fn calculate_length_with_borrowing(s: &String) -> usize {
    // s.push_str(", world"); //this don't compile

    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn push_something(s: &mut String){
    s.push('m');
}

fn using_string_literal(s: &str) -> &str {
    s
}

async fn async_function(x: &mut i32) {
    *x += 1;
    // ... async operations ...
}

// when you have functions that gets something and returns something is "easier"
// because you pass and return a value, so you can use the new one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


#[cfg(test)]
mod tests {
    // Import necessary items from the code being tested
    use super::*;

    // Test function with the #[test] attribute
    #[test]
    fn test_ownership_playground() {
        // Arrange
        ownership_playground();
    }
}
