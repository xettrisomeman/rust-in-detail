
//using standard library function mem
use std::mem;

//array

fn main(){
    let xs: [i32; 3] = [1,2,4]; //array of interger 32bit total 3 value
    println!("(First Element is {}) . {:?} and its length is {}",xs[0],xs,xs.len()); //print 1st value
    println!("Size of the array is {}", mem::size_of_val(&xs));//using std library std:: 
    //slice
    println!("{:?} is element 1st and 2nd", &xs[0..2]); //&--> it is reference , we are referencing xs here
    println!("Total elements in the array {:?}",&xs[0..=2]); //=2 says print the 2nd value also

    
}


