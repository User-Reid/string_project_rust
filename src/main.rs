use std::io;

fn make_money(x: &mut String) {
    let y = format!("{x}$$$");
    println!("{y}");
}

fn trim_and_capitalize(x: &str) -> String {
    x.trim().to_uppercase()
}

fn elements(x: &str) -> Vec<&str> {
    x.split("!").collect()
}

fn get_identity() -> String {
    let mut x: String = String::new();
    let mut y: String = String::new();
    let input = io::stdin();
    println!("What is your first name bitch?");
    input.read_line(&mut x).expect("There was an issue getting your first name");
    println!("What is your last name, {x}");
    input.read_line(&mut y).expect("There was an issue getting your last name dummy");
    println!("Well howdy there {0} {1}😀.", x.trim(), y.trim());
    format!("{0} {1}", x.trim(), y.trim())
}

fn main() {
    let mut a: String = String::from("Potato");
    make_money(&mut a);
    println!("{}", trim_and_capitalize("     Brocoli and chips    "));

    println!("{:#?}", elements("Brandon!Kyle!Albert!Reid!Rae"));

    let name = get_identity();
    println!("{name}")
}