fn main() {
    let mut wishes: i32 = 3;
    println!("I wish for 4 wishes");
    let amount_of_added_wishes: i32 = 4;
    wishes -= 1;
    println!("You now have {wishes} wishes left");
    println!("Wish granted");
    wishes = amount_of_added_wishes;
    println!("You now have {} wishes", wishes);

    // This is due to the fact that: you wished for 4 wishes, it does not mean geni adds 1 wish to
    // ur total wishes, the geni replaces the current amount of wishes u have into 4,
    // the geni accepts ur request, subtracts 1 wish(now u have 2), and gives u a total of 4 (replaces current amount of wishes into 4),
    // he does not do x+1,
    // he does x = 4.
}
