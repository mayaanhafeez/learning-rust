fn main() {
    let x = 5;
    println!("the value of x is {x}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in this scope is {x}");
    }

    println!("the value of x is {x}");
}
