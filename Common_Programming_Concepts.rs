fn main() {
    /*let mut x = 5;
    println!("The Value of X: {x}");
    x = 6;
    print!("The Value of x: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;*/
    
    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("The of X in the inner scope is:{x}")
    }
    println!("The value of x is: {x}");
}
