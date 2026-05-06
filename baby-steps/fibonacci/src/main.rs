use std::io;

fn fibo() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Oops!, Failed to read input.");
    
    let str_format: String = match input.trim().parse() {
        Ok(str_i) => str_i,
        Err(_) => "".to_string()
    };
    if str_format == "Explain".to_string() {
        println!("\nThe Fibonacci sequence is a series of numbers where \
            each number (Fibonacci number) is the sum of the two preceding ones, \
            typically starting from 0 and 1. It is is also known for its frequent appearance in nature\n");
    } else {
        let num: u32 =  match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Positive Integer required!");
                0
            }
        };
        let mut output_str = String::new();
        let mut cache: [usize; 2] = [0, 1]; 

        for i in 0..(num + 1) as usize {
            if i <= 1 {
                output_str.push_str(&i.to_string());
                output_str.push_str(", ");
            } else {
                let cv = match cache[0].checked_add(cache[1]) {
                    Some(result) => result,
                    None => {
                        println!("Number too large! Sequence results into a computation error!");
                        return;
                    }
                };
                cache[0] = cache[1];
                cache[1] = cv;
                output_str.push_str(&cv.to_string());
                output_str.push_str(", ");
            }
        }
        output_str = output_str.trim_end_matches(", ").to_string();
        println!("Fibonacci sequence starting from F0-F{num}: {}", output_str)
    }
}

fn main() {
    println!("Hello there, generate a Fibonacci \
        sequence for a number of your choice!.\nInput 'Explain' \
        to get a view of what Fibonacci is");
    fibo();
}
