## Rust ownership by example

Rust's data ownership model enforces strict rules on ownership, borrowing, and lifetimes to guarantee memory safety and prevent data races without the need for a garbage collector.

### Summary

Borrowing allows a function to access data by reference mutable and immutable, rather than owning it. To prevent data races at any given time, either multiple immutable references to data or a single mutable reference can exist. 

Lifetimes, denoted by a `'a` syntax, are annotations that specify how long references to data should remain valid. They ensure that references don't outlive the data they point to, preventing "dangling references." The Rust compiler uses lifetimes to enforce these rules at compile time, ensuring that references are always valid and don't lead to undefined behavior.


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

## Concurrency


Rust's borrowing and lifetime rules become especially intricate when dealing with concurrency. Here are some common pitfalls, especially in the context of concurrent programming:

1. **Mutable Alias in Concurrency**

   **Pitfall**: Mutable references in threads can lead to data races.

   ```rust
   use std::thread;
   
   let mut data = vec![1, 2, 3];

   thread::spawn(move || {
       data[0] += 1;  // Error: Can't use `data` here
   }).join().unwrap();

   println!("{:?}", data);
   ```

   **Solution**: Use atomic types, Mutexes, or other synchronization primitives provided by Rust's standard library.

   ```rust
   use std::thread;
   use std::sync::{Arc, Mutex};

   let data = Arc::new(Mutex::new(vec![1, 2, 3]));

   let data_clone = Arc::clone(&data);
   thread::spawn(move || {
       let mut data = data_clone.lock().unwrap();
       data[0] += 1;
   }).join().unwrap();

   println!("{:?}", *data.lock().unwrap());
   ```

2. **Long-lived Threads with Short-lived References**

   **Pitfall**: Using a reference in a thread that may outlive the data it refers to.

   ```rust
   use std::thread;

   let value = "hello".to_string();
   let reference = &value;

   thread::spawn(move || {
       println!("{}", reference);  // Error: reference may outlive data it refers to
   });
   ```

   **Solution**: Ensure that data lives long enough, or pass ownership to the thread.

   ```rust
   use std::thread;

   let value = "hello".to_string();

   thread::spawn(move || {
       println!("{}", value);
   }).join().unwrap();
   ```

3. **Lifetimes with `RwLock`**

   **Pitfall**: Holding a read-write lock's read guard and then trying to get a write guard.

   ```rust
   use std::sync::{RwLock, Arc};
   use std::thread;

   let lock = Arc::new(RwLock::new(5));

   let r_guard = lock.read().unwrap();
   let w_guard = lock.write().unwrap();  // Deadlock!
   ```

   **Solution**: Drop the read guard before acquiring the write guard.

   ```rust
   use std::sync::{RwLock, Arc};
   use std::thread;

   let lock = Arc::new(RwLock::new(5));

   {
       let r_guard = lock.read().unwrap();
       // Use r_guard
   }  // r_guard goes out of scope here

   let w_guard = lock.write().unwrap();
   ```
