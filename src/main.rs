use std::io;

fn main() {

    loop {
        println!("Enter a number!");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The {user_input}th fibonacci number is {}", fib(user_input));
        break;

    }
    

}

fn fib(n: u32) -> u32 { 
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}
