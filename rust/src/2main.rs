//fn main() {
//  println!("Diw");
//  print!("");
//}
fn main() {
    println!("Sigma.Rust is best");
}

// e
//fn /main() {
//    let name = "John";
//  println!("Guess the word!");
//  let mut guess: String = /*String::new()*/"hello".to_string();
//let mut guess = String::new();
/* std::io::stdin()
    .read_line(&mut guess)  //THIS WILL NOT OVERIDE THE VARIABLE CONNET IT WILLADD NEW THNGS TO IT
    // EG: FI THE VAR HAD: 63, AND U WROTE "FIRE"  IT WOULD OUTPUT 63FIRE
    .expect("Failed to read input");
 //   println!("You guessed: {} {}", guess.trim(), name.trim());
   println!("You guessed: {}", guess.trim() );

}
 */

/*
use std::io;:;


fn main(){
println!("Hi");
let mut guess = String::new();
io::stdin().read_line(&mut guess).expect("Error Occurecd whil reading The input");
println!("You guessed: {}", guess.trim());


}
*/
/*
let x = 60;
x = x + 10; // This line will cause a compile-time error because `x` is immutable


let mut x = 60;
x = x + 10; // This is valid because `x` is mutable



const MAX_POINTS: u32 = 100_000;
we have to declare the type of a constant
it is immutable by default and per convention, constants are written in uppercase with underscores
and it must be annotated with a type
and it can be declared in any scope, including the global scope
and it cant be set to mutable
can constants be mutable?
?No, constants cannot be mutable in Rust. They are always immutable.










let apples = 5;
rust variable is immutable by default
*/

/*

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
fn main() {
    let app = Application::builder()
        .application_id("com.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hello World")
            .default_width(300)
            .default_height(100)
            .build();

        window.show();
    });

    app.run();
}*/

// let mut x = 5; // x IS mutable
// x = 6; // THIS IS VALID BECAUSE X IS MUTABLE
// println!("The value of x is: {}", x);
// IS const immutable
// cargo check
