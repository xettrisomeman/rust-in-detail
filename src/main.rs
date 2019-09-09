
//string in rust
//String is stored in heap same like array and list
//so we have to use & whenerver we are doing somthing with string


fn main(){

    let s = "String".to_owned(); //slice of string , it is not string but a str
    let ss = String::from(s); //this is now a string

    println!("{}",ss);
    //cannot use s here because it it already used in ss
    
    // //it will cause an error
    // println!("{}",s );


    //string slicing
    let slice = &ss[1..3];
    println!("{}",slice );
    

    //string concatation
    // let sss = String::from("Hello");
    // let concat = sss + &ss; //we have to provide a reference to string whenever we are concating the string
    
    //printing concated string
    // println!("{}",concat );

    //new str
    let mut ssss = String::from("Hello");
    //pushing str
    ssss.push_str("World");
    println!("{}",ssss );

    }

