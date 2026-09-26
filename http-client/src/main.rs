use std::io::{Write, stdin, stdout};
use anyhow::Result;

// disable smart view probably before running this lol, just got flagged? nice one. 2nd try it let it through, what the f**k?

//error: could not execute process `target\debug\http-client.exe` (never executed)
//Caused by:
//  An Application Control policy has blocked this file. (os error 4551) , not joking.

#[tokio::main]
async fn main() -> Result<()>{
    print!("Enter domain: ");
    stdout().flush()?;

    let mut domain = String::new();
    stdin().read_line(&mut domain)?;
    let domain = domain.trim();

    let url = format!("http://{domain}:80"); // if you can figure out how to make it automatically formatted into a url, would be amazing, im getting hungry and impatient
   
    let request = reqwest::get(&url).await?;
    print!("Status: {}", request.status());
    Ok(())
}
