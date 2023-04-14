use restson::*;

pub mod rumpus;
use rumpus::types::*;
use rumpus::query::*;

const RUMPUS_URL: &str = "https://www.bscotch.net/api/";
const API_KEY: &str = include_str!("../key.txt");

#[tokio::main]
async fn main() {
	let mut client = RestClient::new(RUMPUS_URL).unwrap();
	client.set_header("Rumpus-Delegation-Key", API_KEY).expect("api key is not a valid HTTP header");
	
	let search = PlayerSearch::new()
		.user_ids("m7n6j8,pg11x1")
		.include_aliases(true);
	
	let data = client.get::<_, Rumpus<Vec<Player>>>(search).await;
	let _ = dbg!(data);
}
