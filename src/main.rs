use std::io;

#[tokio::main]
async fn main(){
    let args_ = args();
    let full_url: &str = &format!("https://img.shields.io/badge/{}",args_)[..];
    //println!("{}",full_url);
}

fn args () -> String {
    println!("\nLabel name: ");
    let mut labelAsk = String::new();
    io::stdin()
    .read_line(&mut labelAsk)
    .expect("Failed to read line");

    println!("\nMessage: ");
    let mut messageAsk = String::new();
    io::stdin()
    .read_line(&mut messageAsk)
    .expect("Failed to read line");

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