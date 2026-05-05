fn const_() {
    //const SAT: String = String::new();
    const SAT: u32 = 23;
    println!("Hello {SAT}\n");
}

fn let_() {
    let spaces = "    ew";
    println!("Spaces are: {spaces} ");

    {
        let spaces = "redefined";
        println!("Spaces are: {spaces} ");
    }

    println!("Spaces are: {spaces} ");
}

fn main() {
    println!("Const examples \n----------- ");
    const SAT: &str = "dsds";
    println!("Hello {SAT}");
    const_();
    println!("Let redefinition examples \n----------- ");
    let_();
}
