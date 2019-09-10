
#[derive(Debug)] //this helps to use trait

//struct

//creating a struct object
struct Object {
    width: u32,
    height: u32
}

// //creating a function which will hold obj variable of Object
// fn area(obj: &Object) -> u32{
//     //to access struct
//     obj.width * obj.height
//     //return the multiplication of obj.width and obj.height
// }

impl Object {
    //self means init to Object 
    fn area(&self) -> u32{
        self.height * self.width
    }

    fn show(&self){
       println!("{} * {} is {}",self.height,self.height ,self.area() );
    }

}

//impl object for creating new object
impl Object {
    fn new(width: u32 , height: u32) -> Object {
        Object{
            width,
            height,
        }
    }
}



fn main() {
    //defining value here
    let object: Object = Object{
        width:100,
        height:100
    };

    let obj = Object::new(11, 102); //creating new object 

    //printing the are
    println!("The area of the object is {}", object.area()); //passing value to the area as a reference
    println!("The area of the object is {}", obj.area()); //passing value to the area as a reference   
    obj.show();
    object.show();

    println!("{:#?}", obj);
    println!("{:#?}", object);
}


