



// fn main() {
//     let x = 15;


//     match x{
//         2|4|6|8 => println!("This is a even"),
//         1|3|5|7 => println!("This is a odd"),
//         9..=16 => println!("This is in between 9 and 14"),
//         _ => println!("OH MAI GHAAAH"),
//     }
// }


// fn main() {
//     let pair = (1,2);

//     match pair {
//         (0,y)=> println!("y: {}",y ),
//         (x,0)=> println!("x: {}",x ),
//         _ => println!("Excuse me?")
//     }
// }


fn main() {
    let pair = (5 ,5);

    match pair{
        (x,y)if x==y => println!("X and Y are both equal"),
        (x,y) if x > y => println!("X is greater than Y"),
        (x , _) if x % 2 ==0 => println!("X is even"),
        _ => println!("What is happening with you?")
    }
}