fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci generator");
    let num1: u32 = 0;
    let num2: u32 = 1;
    let num3: u32 = 10; 
    println!("{}", fibonacci(num1));
    println!("{}", fibonacci(num2));
    println!("{}", fibonacci(num3));
}