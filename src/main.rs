


fn main(){
   //array vs tuple
   let array : [i32;4] = [1,2,3,4]; //same data types
   let tuple: (i32, char, f64) = (1,'w',2.5); //different data types

   println!("{}",tuple.1 ); //call it using index
   println!("{}",array[0] ); //call it using method
   //or use destructure method

   let (x,y,z) = tuple;

   println!("{}", x); //1

   println!("{} Is addition between tuple and array ",add(x,array[0]));
   println!("{} is unusal addition of tuple and array",add2(x, array[1] as i64) ); //passing value to ffunction
   //because  it accepts i64
}


//new function
//return a+b
//->means return 
fn add(a:i32 , b:i32) -> i32{
    a+b
}

//new unusual function
//now we have b as i64 but returning value is i32
fn add2(a:i32, b:i64) -> i32{
    a+ (b as i32) //so we change b to be i32
    //b is set to (i32) because this function returns value as i32
}

