## Rust ownership by example

Rust's data ownership model enforces strict rules on ownership, borrowing, and lifetimes to guarantee memory safety and prevent data races without the need for a garbage collector.

### Summary

Borrowing allows a function to access data by reference mutable and immutable, rather than owning it. To prevent data races at any given time, either multiple immutable references to data or a single mutable reference can exist. 

Lifetimes, denoted by a 'a syntax, are annotations that specify how long references to data should remain valid. They ensure that references don't outlive the data they point to, preventing "dangling references." The Rust compiler uses lifetimes to enforce these rules at compile time, ensuring that references are always valid and don't lead to undefined behavior.


The following are the most common pitfalls related to ownership, borrowing, and lifetimes. 

1. **Borrowing after Moving**

   **Pitfall**: Once a value has been moved, it can't be used again.
   
   ```rust
   let s1 = String::from("hello");
   let s2 = s1;
   println!("{}", s1); // Error!
   ```

   **Solution**: Clone the data if needed, or use references.
   
   ```rust
   let s1 = String::from("hello");
   let s2 = s1.clone();
   println!("{}", s1); // No error
   ```

2. **Mutable and Immutable Borrows**

   **Pitfall**: You can't have a mutable reference while you have an immutable one.
   
   ```rust
   let mut s = String::from("hello");
   let r1 = &s;
   let r2 = &mut s; // Error!
   ```

   **Solution**: Restructure code to ensure that mutable and immutable references don't overlap.
   
   ```rust
   let mut s = String::from("hello");
   let r1 = &s;
   println!("{}", r1);
   let r2 = &mut s;
   ```

3. **Dangling References**

   **Pitfall**: Returning a reference to data that goes out of scope can lead to a dangling reference.

   ```rust
   fn dangle() -> &String {
       let s = String::from("hello");
       &s // Error!
   }
   ```

   **Solution**: Return the owned data type or ensure the referenced data lives long enough.
   
   ```rust
   fn no_dangle() -> String {
       let s = String::from("hello");
       s
   }
   ```

4. **Lifetimes Misunderstanding**

   **Pitfall**: Not understanding how lifetimes work can lead to borrow checker errors.
   
   ```rust
   fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
       if s1.len() > s2.len() {
           s1
       } else {
           s2
       }
   }
   
   let s1 = String::from("long string is long");
   {
       let s2 = String::from("xyz");
       let result = longest(s1.as_str(), s2.as_str()); // Error!
       println!("longest string is {}", result);
   }
   ```

   **Solution**: Understand that the function signature promises that the returned reference will live as long as the shortest of its input lifetimes. Restructure code or use different lifetime annotations.

   ```rust
   fn longest<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
       if s1.len() > s2.len() {
           s1
       } else {
           &s1[0..0] // return an empty slice of s1
       }
   }
   ```

5. **Holding References in Structs**

   **Pitfall**: Storing references in structs without lifetimes.

   ```rust
   struct Book {
       title: &str, // Error!
   }
   ```

   **Solution**: Use lifetimes to indicate the relationship between the struct and the reference.
   
   ```rust
   struct Book<'a> {
       title: &'a str,
   }
   ```

These are just a few examples. The key to navigating Rust's ownership model is to practice and understand the rules and guarantees it provides. The borrow checker will always be there to catch potential issues, helping developers to internalize these concepts over time.