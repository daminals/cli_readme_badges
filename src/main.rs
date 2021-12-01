use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let resp = reqwest::get("https://shields.io/endpoint?")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

fn args () -> &'static str {
    let SchemaVersion = "1";
    let label = "temp";
    let color = "temp";

    return format!("SchemaVersion={}&label={}&color={}", SchemaVersion, label, color);
}