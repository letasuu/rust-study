use std::io;

fn main(){

    println!("Hello, {}",change_name());

}

fn change_name() ->String{

    let mut name = String::new();

    println!("enter your name!");

    io::stdin()
        .read_line(&mut name)
        .expect("ybaiyo,yabaiyo");

    let mut keisho = String::new();

    println!("tell me keisho!");

    io::stdin()
        .read_line(&mut keisho)
        .expect("ybaiyo,yabaiyo");

    let new_namae = name.trim_end().to_owned() + " " + &keisho.trim_end().to_owned() + "!";
    new_namae

}
