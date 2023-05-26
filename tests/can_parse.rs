use restson::RestClient;

use rumpus_ce::{
	types::*,
	query::*,
};

use anyhow::{Error, Result};

const API_KEY: &str = include_str!("../key.txt");

fn client() -> RestClient {
	rumpus_ce::rest_client::with_key(API_KEY)
}

fn err_info<R>(res: Result<R, restson::Error>) -> Result<R, restson::Error> {
	match res {
		Ok(res) => Ok(res),
		Err(err) => {
			use restson::Error::*;
			if let DeserializeParseError(ref e, ref s) = err {
				assert_eq!(e.line(), 1);
				let col = e.column();
				let mut char_i = s.char_indices();
				let from = char_i.nth(col.saturating_sub(40)).expect("column not in string").0;
				let to = char_i.nth(80).unwrap_or_else( ||
					s.char_indices().last().expect("string didn't have characters")
				).0;
				eprintln!("{}", &s[from..to]);
			}
			Err(err)
		}
	}
}

///Test we can fetch & parse info about the current delegation key
#[tokio::test]
async fn this_key() {
	let res = client().get::<_, Rumpus<DelegationKeyInfo>>(()).await;
	let res = err_info(res);
	assert!(matches!(res, Result::Ok(_)));
}

///Verify we can fetch and parse the newest 16 players
#[tokio::test]
async fn newest() -> Result<(), Error> {
	let search = PlayerSearch::new()
		.sort(PlayerSortProperty::CreatedAt, false)
		.limit(16)?
		.include_aliases(true)
		.include_my_interactions(true);
	
	let res = client().get::<_, Rumpus<Vec<Player>>>(search).await;
	let res = err_info(res)?;
	let data = res.into_inner().data.expect("no data was returned");
	
	//Verify the API returned stuff that will have gotten parsed
	assert_eq!(data.len(), 16);
	
	Ok(())
}

///Verify we can fetch and parse the oldest 16 players
#[tokio::test]
async fn oldest() -> Result<(), Error> {
	let search = PlayerSearch::new()
		.sort(PlayerSortProperty::CreatedAt, true)
		.limit(16)?
		.include_aliases(true)
		.include_my_interactions(true);
	
	let res = client().get::<_, Rumpus<Vec<Player>>>(search).await;
	let res = err_info(res)?;
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
		.user_ids(vec!["0ihetl","8mbjmz","pg11x1","bscotch246"])?
		//Make sure the return order is stable
		.sort(PlayerSortProperty::CreatedAt, true)
		.include_aliases(true)
		.include_my_interactions(true);
	
	let res = client().get::<_, Rumpus<Vec<Player>>>(search).await;
	let res = err_info(res)?;
	let data = res.into_inner().data.expect("no data was returned");
	
	// dbg!(&data);
	
	let mut i = 0;
	
	// This is the oldest one with -1 shoes
	assert_eq!(data[i].user_id, "0ihetl");
	assert_eq!(data[i].stats.shoes, -1);
	i+=1;
	
	//This one has -2 published levels	
	assert_eq!(data[i].user_id, "bscotch246");
	assert_eq!(data[i].stats.published, -2);
	i+=1;
	
	//This one also has a negative number of crowns, and DBComp/d_b_comp
	assert_eq!(data[i].user_id, "8mbjmz");
	assert_eq!(data[i].stats.shoes, -1);
	assert_eq!(data[i].stats.crowns, -1);
	assert_eq!(data[i].stats.d_b_comp, Some(3));
	i+=1;
	
	//This one appears to have been deleted
	assert_eq!(data[i].user_id, "pg11x1");
	assert!(matches!(
		data[i].alias,
		Some(Alias {
			alias_type: None,
			anonymous: Some(true),
			alias: None,
			user_id: _
		})
	));
	
	Ok(())
}

///Verify we can fetch and parse the newest levels
#[tokio::test]
async fn newest_levels() -> Result<(), Error> {
	let search = LevelSearch::new()
		.include_aliases(true)
		.include_beta(true)
		.include_my_interactions(true)
		.include_records(true)
		.include_stats(true)
		.limit(64)?
		.sort(LevelSortProperty::CreatedAt, false);
	
	let res = client().get::<_, Rumpus<Vec<Level>>>(search).await;
	let res = err_info(res)?;
	let data = res.into_inner().data.expect("no data was returned");
	
	//Verify the API returned stuff that will have gotten parsed
	assert_eq!(data.len(), 64);
	
	Ok(())
}

///Verify we can fetch and parse the newest levels
#[tokio::test]
async fn oldest_levels() -> Result<(), Error> {
	let search = LevelSearch::new()
		.include_aliases(true)
		.include_beta(true)
		.include_my_interactions(true)
		.include_records(true)
		.include_stats(true)
		.limit(64)?
		.sort(LevelSortProperty::CreatedAt, true);
	
	let res = client().get::<_, Rumpus<Vec<Level>>>(search).await;
	let res = err_info(res)?;
	let data = res.into_inner().data.expect("no data was returned");
	
	//Verify the API returned stuff that will have gotten parsed
	assert_eq!(data.len(), 64);
	
	Ok(())
}