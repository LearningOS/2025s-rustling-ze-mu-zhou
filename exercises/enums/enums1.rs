// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    Quit, 
    Echo(i32), 
    Move , 
    ChangeColor ,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(1));
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
