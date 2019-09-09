//ownership


// fn main() {
//     //assign a to array
//     let a = [1, 2, 3];
//     //let b is equal to b
//     let b = a;
//     println!("{:?} {:?}", a, b); // [1, 2, 3] [1, 2, 3]

//     //it wont change the value
//     //because list and array are stored in stack

// }

//NOW LETS DO SAME LIKE ABOVE BUT WITH HEAP DATA

// fn main(){
//     let a = vec![1,2,3];
//     let b =a;
//     println!("{:?}",a );
//     //it will cause an error

//     //because data stored in heap can be assigned again
//     //it will cause owner problem 
// }

//NOW WE SHOULD DO BORROWING

// There are two types of Borrowing,

//     Shared Borrowing (&T)
//     ▸ A piece of data can be borrowed by a single or multiple users, but data should not be altered.
//     Mutable Borrowing (&mut T)
//     ▸ A piece of data can be borrowed and altered by a single user, but the data should not be accessible for any other users at that time.

// ⭐️ And there are very important rules regarding borrowing,

//     One piece of data can be borrowed either as a shared borrow or as a mutable borrow at a given time. But not both at the same time.
//     Borrowing applies for both copy types and move types.
//     The concept of Liveness ↴


fn main() {
  let mut a = vec![1, 2, 3];
  let b = &mut a;  //  &mut borrow of a starts here
  // some code
  
  println!("{:?}", a); // trying to access a as a shared borrow, so giving error
  main2();
}                  //  &mut borrow of a ends here

//WE CAN ALSO DO THIS


fn main2() {
    let mut a = [1, 2, 3];
    let b = &mut a;
    b[0] = 4;
    println!("{:?}", b); // [4, 2, 3]
}

/*
In Rust,
▸ A resource can only have one owner at a time. When it goes out of the scope, Rust removes it from the Memory.
▸ When we want to reuse the same resource, we are referencing it/ borrowing its content.
▸ When dealing with references, we have to specify lifetime annotations to provide instructions for the compiler to set how long those referenced resources should be alive.
▸ ⭐️But because of lifetime annotations make code more verbose, in order to make common patterns more ergonomic, Rust allows lifetimes to be elided/omitted in fn definitions. In this case, the compiler assigns lifetime annotations implicitly.
*/

