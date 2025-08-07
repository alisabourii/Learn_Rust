use std::io;

fn main() {
    //-------- Variables and Mutability -----------
    /*let mut x = 5;
    println!("The Value of X: {x}");
    x = 6;
    print!("The Value of x: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;*/
    
    /*let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("The of X in the inner scope is:{x}")
    }
    println!("The value of x is: {x}");*/

    /*
    let spaces = "1 3";
    let spaces = spaces.len();
    println!("The value of xspaces is: {spaces}");*/


    //------------- Data Types -----------
    
    //let guess = "42".parse().expect("Not a numeric value");

    /*let x=2.0;
    let y=3.0;
    println!("The value of X is: {x}");
    println!("The value of Y is: {y}");*/

    //Addition
    /*let sum = 5+10;

    //Subtraction
    let diffrent = 95.5-4.3;

    //multiplication
    let product = 4*30;

    //division
    let quotient = 56.7/32.2;
    let truncated = -5/3;

    //Reminder
    let remaindier = 43%3; */

    //------- The Boolean Type -------
    let t=true;

    let f=false;

    //-------- The Tuple Type--------
    let tup = (500,6.4,1);
    let (x,y,z) = tup;
    //println!("The Value of y is: {y}");

    //-------- The Array Type--------
    let a = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a = [3; 5];

    let firstMonth = months[0];
    let lastMonth = months[11];
    //println!("First Month : {firstMonth}");
    //println!("Last Month : {lastMonth}");

    //foundArrayIndex();
    another_func(5);

}

fn foundArrayIndex(){
    let a = [1,2,3,4,5];
    println!("Enter Your Array Index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!!");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index enterd was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}")

}

fn another_func(x:i32){
    print!("The Value of X: {x}");
}
