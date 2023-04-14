use serde::{Serialize, Deserialize};
use restson::*;

///Wrapper for the generic response data all endpoints return
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Rumpus<D> {
	pub data: Option<D>,
	// pub meta: HashMap<???>,
	pub message: Option<String>,
	pub location: Option<String>,
	// pub errors: Vec<???>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct DelegationKeyThis {
	user_id: String,
	pass_id: String,
	permissions: Vec<String>,
}
impl RestPath<()> for Rumpus<DelegationKeyThis> {
	fn get_path(_:()) -> Result<String, Error> {
		Ok(String::from("delegation/keys/@this"))
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Alias {
	user_id: String,
	alias: String,
	anonymous: bool,
	context: String,
}

type Stat = u32;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct PlayerStats {
	subscribers: Stat,
	published: Stat,
	plays: Stat,
	play_time: Stat,
	crowns: Stat,
	shoes: Stat,
	levels_played: Stat,
	wins: Stat,
	fails: Stat,
	num_following: Stat,
	///Shown in the example in the documentation, but not returned
	d_b_comp: Option<Stat>,
	///Tower trials completed, irregardless of if the time trophy was acquired
	#[serde(rename="ChalWins")]
	tower_trials: Stat,
	///Tower trial time trophies acquired
	time_trophies: Stat,
	fave_gen: Stat,
	like_gen: Stat,
	bucks_tipped: Stat,
	tips_gotten: Stat,
	///Shown in the example in the documentation, but not returned
	ach_points: Option<Stat>,
	campaign_prog: Stat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PlayerInteractions {
	following: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Player {
	#[serde(rename="_id")]
	id: String,
	user_id: String,
	alias: Option<Alias>,
	created_at: String,
	updated_at: String,
	interactions: Option<PlayerInteractions>,
	stats: PlayerStats,
}
//Not the full capabilities of this endpoint
impl RestPath<&str> for Rumpus<Vec<Player>> {
	fn get_path(user_code: &str) -> Result<String, Error> {
		Ok(format!("levelhead/players?userIds={}", user_code))
	}
}