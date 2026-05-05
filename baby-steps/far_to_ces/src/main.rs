use std::io;

fn far_to_ces() {
    println!("Input your value:\n");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Oops! Input couldn't be read. Try again");

    let input: f32 = match input.trim().parse() {
       Ok(num) => {
           println!("");
           num
       },
       Err(_) => {
           println!("Input Integers only!!\n");

           return
       }
    };
    
    let ces = if input > 0.0 {(input as f32 - 32.0) * (5.0/9.0)}
                else {0.0};

    println!("Your value in celsius is {ces}\n\nThanks for using our software!!");
}

fn main() {
    println!("Hello there, This is your handy \
        farenheit to celsius converter.\n--------");
    far_to_ces();
}
