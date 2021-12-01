use std::io;

fn main(){
    let full_args = args();
    let args_0 = full_args.0;
    let full_url: &str = &format!("https://img.shields.io/badge/{}",args_0)[..];
    let md_form = markdown_format(full_url, full_args.1);
    linebreak();
    println!("\n{}",md_form);
}

fn args () -> (String, String) {
    let mut labelAsk = ask_input("Label: "); 
    let mut messageAsk = ask_input("Message: ");
    let mut colorAsk = ask_input("Color: ");

    let label: &str = labelAsk.trim();
    let message: &str = messageAsk.trim();
    let color: &str = colorAsk.trim();

    return (format!("{}-{}-181717?color={}",label,message,color), label.to_owned());
}

fn markdown_format(link: &str, alt_text: String) -> String {
    return format!("![github repo badge: {}]({})", alt_text.replace("%20"," "), link);
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