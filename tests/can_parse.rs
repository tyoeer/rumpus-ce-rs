use restson::RestClient;

use rumpus_ce::{
	types::*,
	query::*,
};

use anyhow::*;

const API_KEY: &str = include_str!("../key.txt");

fn client() -> RestClient {
	let mut client = RestClient::new(rumpus_ce::API_URL).expect("Rumpus url wasn't valid");
	client.set_header("Rumpus-Delegation-Key", API_KEY).expect("api key is not a valid HTTP header");
	client
}

///Test we can fetch & parse info about the current delegation key
#[tokio::test]
async fn this_key() {
	let res = client().get::<_, Rumpus<DelegationKeyInfo>>(()).await;
	assert!(matches!(res, Result::Ok(_)));
}

///Verify we can fetch and parse the newest 16 players
#[tokio::test]
async fn newest() -> Result<(), Error> {
	let search = PlayerSearch::new()
		.sort(SortProperty::CreatedAt, false)
		.limit(16)?
		.include_aliases(true)
		.include_my_interactions(true);
	
	let res = client().get::<_, Rumpus<Vec<Player>>>(search).await?;
	let data = res.into_inner().data.expect("no data was returned");
	
	//Verify the API returned stuff that will have gotten parsed
	assert_eq!(data.len(), 16);
	
	Ok(())
}

///Verify we can fetch and parse the oldest 16 players
#[tokio::test]
async fn oldest() -> Result<(), Error> {
	let search = PlayerSearch::new()
		.sort(SortProperty::CreatedAt, true)
		.limit(16)?
		.include_aliases(true)
		.include_my_interactions(true);
	
	let res = client().get::<_, Rumpus<Vec<Player>>>(search).await?;
	let data = res.into_inner().data.expect("no data was returned");
	
	//Verify the API returned stuff that will have gotten parsed
	assert_eq!(data.len(), 16);
	
	// Might as well run some additional checks
	assert_eq!(data[0].id, "5c7715223116090016409e56");
	assert_eq!(data[0].user_id, "bscotch119");
	
	Ok(())
}

///Test that we can parse some special players, and that their special attributes still hold
#[tokio::test]
async fn special() -> Result<(), Error> {
	let search = PlayerSearch::new()
		.user_ids(vec!["0ihetl","8mbjmz","pg11x1"])?
		//Make sure the return order is stable
		.sort(SortProperty::CreatedAt, true)
		.include_aliases(true)
		.include_my_interactions(true);
	
	let res = client().get::<_, Rumpus<Vec<Player>>>(search).await?;
	let data = res.into_inner().data.expect("no data was returned");
	
	// dbg!(&data);
	
	// This is the oldest one with -1 shoes
	assert_eq!(data[0].user_id, "0ihetl");
	assert_eq!(data[0].stats.shoes, -1);
	
	//This one also has a negative number of crowns, and DBComp/d_b_comp
	assert_eq!(data[1].user_id, "8mbjmz");
	assert_eq!(data[1].stats.shoes, -1);
	assert_eq!(data[1].stats.crowns, -1);
	assert_eq!(data[1].stats.d_b_comp, Some(3));
	
	//This one appears to have been deleted
	assert_eq!(data[2].user_id, "pg11x1");
	assert!(matches!(
		data[2].alias,
		Some(Alias {
			alias_type: None,
			anonymous: Some(true),
			alias: None,
			user_id: _
		})
	));
	
	Ok(())
}