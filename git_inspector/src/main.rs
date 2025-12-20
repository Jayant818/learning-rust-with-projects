
use futures::future::join_all;
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Debug)]
struct User{
    name:String,
    public_repos:u32,
}

async fn get_user(client:&reqwest::Client,username:&str)->Result<User,reqwest::Error>{
    let url = format!("https://api.github.com/users/{}",username);

    let res = client.get(&url)
        .header("User-Agent", "rust-learning-app")
        .send()
        .await?
        .error_for_status()?;

    let data = res.json::<User>().await?;

    Ok(data)
}

#[tokio::main]
async fn main()->Result<(),reqwest::Error>{
    let usernames = vec!["jayant818","ashishmohapatra240","pallava-Joshi","jassbawa","vivekk-11"];

    let client = reqwest::Client::new();

    let tasks = usernames.iter()
        .map(|u|{
            get_user(&client, u)
        }).collect::<Vec<_>>();

    let result = join_all(tasks).await;

    for res in result {
        match res {
            Ok(user)=> print!("User is {:?}", user),
            Err(err)=>   println!("Failed to fetch : {}", err),
        }   
    }

    Ok(())
}

#[derive(Debug)]
enum RequestError{
    UserDoesNotExist,
}