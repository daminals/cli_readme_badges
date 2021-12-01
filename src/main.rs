use std::io;

fn main(){
    let args_ = args();
    let full_url: &str = &format!("https://img.shields.io/badge/{}",args_)[..];
    let md_form = markdown_format(full_url);
    linebreak();
    println!("\n{}",md_form);
}

fn args () -> String {
    let mut labelAsk = ask_input("Label: "); 
    let mut messageAsk = ask_input("Message: ");
    let mut colorAsk = ask_input("Color: ");

    let label: &str = labelAsk.trim();
    let message: &str = messageAsk.trim();
    let color: &str = colorAsk.trim();
    
    return format!("{}-{}-181717?color={}",label,message,color);
}

fn markdown_format(link: &str) -> String {    
    return format!("![github repo badge]({})", link);
}

fn linebreak () {
    println!("{}", "=".repeat(50));
}

fn ask_input(input: &str) -> String {
    linebreak();
    println!("{}", input);
    let mut user_str = String::new();
    io::stdin()
    .read_line(&mut user_str)
    .expect("Failed to read line");
    user_str = user_str.replace(" ","%20"); // replace the whitespace with escape for whitespace
    
    user_str
}