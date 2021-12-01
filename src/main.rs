use std::io;

fn main(){
    let args_ = args();
    let full_url: &str = &format!("https://img.shields.io/badge/{}",args_)[..];
    let md_form = markdown_format(full_url);
    linebreak();
    println!("{}",md_form);
}

fn args () -> String {
    linebreak();
    println!("\nLabel name: ");
    let mut labelAsk = String::new();
    io::stdin()
    .read_line(&mut labelAsk)
    .expect("Failed to read line");

    linebreak();
    println!("\nMessage: ");
    let mut messageAsk = String::new();
    io::stdin()
    .read_line(&mut messageAsk)
    .expect("Failed to read line");

    linebreak();
    println!("\nColor: ");
    let mut colorAsk = String::new();
    io::stdin()
    .read_line(&mut colorAsk)
    .expect("Failed to read line");    

    let label: &str = labelAsk.trim();
    let message: &str = messageAsk.trim();
    let color: &str = colorAsk.trim();

    return format!("{}-{}-181717?color={}",label,message,color);
}

fn markdown_format(link: &str) -> String {
    linebreak();
    println!("\nAlt Text: ");
    let mut alt_text = String::new();
    io::stdin()
    .read_line(&mut alt_text)
    .expect("Failed to read line");
    let alt_text: &str = alt_text.trim();
    
    return format!("![{}]({})", alt_text, link);
}

fn linebreak () {
    println!("{}", "=".repeat(50));
}