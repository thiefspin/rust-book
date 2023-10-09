use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct DadJoke {
    joke: String,
    status: u8,
}

#[derive(Deserialize, Debug)]
struct TriviaQuestion {
    id: u32,
    question: String,
    answer: String,
    game_id: u32
}

async fn random_trivia_question() -> Result<(), Error> {
    let request_url = format!("http://jservice.io/api/random");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let question: Vec<TriviaQuestion> = response.json().await?;
    println!("{:?}", question);

    let clues_response = reqwest::get(format!("http://jservice.io/api/clues?game_id={game_id}",
                                              game_id = question.get(0).unwrap().game_id)
    ).await?;

    let clues: Vec<TriviaQuestion> = clues_response.json().await?;
    println!("{:?}", clues);

    Ok(())
}

async fn dad_joke() -> Result<(), Error> {
    let request_url = format!("https://icanhazdadjoke.com/");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client.get(&request_url)
        .header("Accept", "application/json")
        .send()
        .await?;
    let joke: DadJoke = response.json().await?;
    println!("{:?}", joke);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    random_trivia_question().await?;
    dad_joke().await?;
    Ok(())
}
