use std::io;

fn main() {
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Fire");
    //TWK THE COLOR OF NAUFAL's HAIRLINE z
    //
    //
    let mut sigma2: String = String::new();
    let mut sigma: String = String::new();
    std::io::stdin()
        //This can be shortern to io::stdin()
        .read_line(&mut sigma)
        .expect("Error occured while reading user input");
    println!("User entered {}", sigma.trim());
    io::stdin()
        .read_line(&mut sigma2)
        .expect("Error occured while reading user input");
    println!("User entered {sigma2}");

    // :: means that this function/lib is associated with the function/lib before ::
    // eg: std::io, meaing the input/output lib in the standard lib, io::stdin, standard input
    // method inside of io, so now u know what std::io::stdin means.
    // if u were to declare use std::io;
    // you would not need to state std::io::stdin(), rather you would need to: io::stdin()
    // btw .read_line and .expect are methods inside of stdin().
    //TWK
    //Also anoter thing, .read_line is a method but for further naming convention info
    //go to rustnaming in the obsidian vault.
    // (RUST ONE)
    println!("Hello, world!");
    println!("Hello,  world!");
}
