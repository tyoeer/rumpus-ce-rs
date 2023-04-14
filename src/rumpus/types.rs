use serde::{Serialize, Deserialize};

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

///Since users have *two* Levelhead aliases (one random, one user-chosen), it's useful to be able to differentiate.
///These are `levelhead` for user-chosen, and `levelhead-safe` for server-chosen aliases.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="kebab-case")]
pub enum AliasType {
	Levelhead,
	LevelheadSafe
}

//Specified at https://www.bscotch.net/api/docs/levelhead/#aliases-alias-reporting-post
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Alias {
	///Synonymous with "Rumpus Lookup Code".
	user_id: String,
	///Doesn't occur when anonymous
	#[serde(rename="context")]
	alias_type: Option<AliasType>,
	///The player's username inside Levelhead. (Not set if the user does not have an alias.)
	alias: Option<String>,
	///If a player's username is not found, this field will exist and be set to `true`.
	///This happens when a user has not yet played Levelhead, or if they've deleted their Rumpus account.
	anonymous: Option<bool>,
}

type Stat = u32;

//Values with #[serde(default)] are not present in the server response when 0
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct PlayerStats {
	subscribers: Stat,
	#[serde(default)]
	published: Stat,
	#[serde(default)]
	plays: Stat,
	play_time: Stat,
	crowns: Stat,
	shoes: Stat,
	#[serde(default)]
	levels_played: Stat,
	#[serde(default)]
	wins: Stat,
	#[serde(default)]
	fails: Stat,
	num_following: Stat,
	///Shown in the example in the documentation, but not returned
	d_b_comp: Option<Stat>,
	///Tower trials completed, irregardless of if the time trophy was acquired
	#[serde(rename="ChalWins", default)]
	tower_trials: Stat,
	///Tower trial time trophies acquired
	#[serde(default)]
	time_trophies: Stat,
	#[serde(default)]
	fave_gen: Stat,
	#[serde(default)]
	like_gen: Stat,
	#[serde(default)]
	bucks_tipped: Stat,
	#[serde(default)]
	tips_gotten: Stat,
	///Shown in the example in the documentation, but not returned
	#[serde(default)]
	ach_points: Option<Stat>,
	///Percentage how much of the campaign/training has been completed
	#[serde(rename="CampaignProg", default)]
	campaign_progress: u8,
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