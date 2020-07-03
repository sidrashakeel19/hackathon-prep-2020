use std::io;

fn main() {
    println!("Choose which format you want to convert from \n Fahrenheit (1) \n Celsius (2)");

    let mut selection = String::new();

    io::stdin().read_line(&mut selection).expect("Failed to read line");
    let selection: u32 = selection.trim().parse().expect("Please type a number!");

    println!("You chose {}",selection);

    temp_conver(selection);
}

fn temp_conver(s: u32) {
    if s == 1 {
        println!("Enter temperature in Fahrenheit");
        
        let mut f1 = String::new();
        io::stdin().read_line(&mut f1).expect("Failed to read line");
        let f1: u32 = f1.trim().parse().expect("Please type a number!");

        let c1 = (f1-32)*5/9;
        println!("Temperature in Celsius is {}", c1);
    }
    else if s == 2 {
        println!("Enter temperature in Celsius");
        
        let mut c2 = String::new();
        io::stdin().read_line(&mut c2).expect("Failed to read line");
        let c2: u32 = c2.trim().parse().expect("Please type a number!");

        let f2 = (c2*9/5)+32;
        println!("Temperature in Fahrenheit is {}", f2);
    }
}