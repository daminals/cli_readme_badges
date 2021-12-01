use std::collections::HashMap;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args_: &str = args();
    let resp = reqwest::get(format!("https://shields.io/endpoint?{}",args_))
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

fn args () -> String {
    println!("Label name: ");
    let mut labelAsk = String::new();
    io::stdin()
    .read_line(&mut labelAsk)
    .expect("Failed to read line");

    println!("Color: ");
    let mut colorAsk = String::new();
    io::stdin()
    .read_line(&mut colorAsk)
    .expect("Failed to read line");    

    let SchemaVersion = "1";
    let label: &str = labelAsk.trim();
    let color: &str = colorAsk.trim();

    return format!("SchemaVersion={}&label={}&color={}", SchemaVersion, label, color);
}