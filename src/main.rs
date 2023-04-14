use restson::*;

mod rumpus;

const RUMPUS_URL: &str = "https://www.bscotch.net/api/";
const API_KEY: &str = include_str!("../key.txt");

#[tokio::main]
async fn main() {
	let mut client = RestClient::new(RUMPUS_URL).unwrap();
	client.set_header("Rumpus-Delegation-Key", API_KEY).expect("api key is not a valid HTTP header");
	let data = client.get::<_, rumpus::Rumpus<Vec<rumpus::Player>>>("m7n6j8").await;
	dbg!(data);
}
