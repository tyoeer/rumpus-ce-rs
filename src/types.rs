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
pub struct DelegationKeyInfo {
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
///A player's name
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Alias {
	///Synonymous with "Rumpus Lookup Code".
	pub user_id: String,
	///Doesn't occur when anonymous
	#[serde(rename="context")]
	pub alias_type: Option<AliasType>,
	///The player's username inside Levelhead. (Not set if the user does not have an alias.)
	pub alias: Option<String>,
	///If a player's username is not found, this field will exist and be set to `true`.
	///This happens when a user has not yet played Levelhead, or if they've deleted their Rumpus account.
	pub anonymous: Option<bool>,
}

/**
Used for numbers with an unknown upper bound.

An i32 because multiple players such as <https://lvlhd.co/@8mbjmz> managed to get -1 shoes and/or crowns
*/
pub type Stat = i32;

///Various statistics about a player
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct PlayerStats {
	//Values with #[serde(default)] are not present in the server response when 0
	
	pub subscribers: Stat,
	#[serde(default)]
	pub published: Stat,
	#[serde(default)]
	pub plays: Stat,
	pub play_time: Stat,
	pub crowns: Stat,
	pub shoes: Stat,
	#[serde(default)]
	pub levels_played: Stat,
	#[serde(default)]
	pub wins: Stat,
	#[serde(default)]
	pub fails: Stat,
	pub num_following: Stat,
	pub d_b_comp: Option<Stat>,
	///Tower trials completed, irregardless of if the time trophy was acquired
	#[serde(rename="ChalWins", default)]
	pub tower_trials: Stat,
	///Tower trial time trophies acquired
	#[serde(default)]
	pub time_trophies: Stat,
	#[serde(default)]
	pub fave_gen: Stat,
	#[serde(default)]
	pub like_gen: Stat,
	#[serde(default)]
	pub bucks_tipped: Stat,
	#[serde(default)]
	pub tips_gotten: Stat,
	///Shown in the example in the documentation, but not returned
	#[serde(default)]
	pub ach_points: Option<Stat>,
	///Percentage how much of the campaign/training has been completed
	#[serde(rename="CampaignProg", default)]
	pub campaign_progress: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PlayerInteractions {
	pub following: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Player {
	#[serde(rename="_id")]
	pub id: String,
	pub user_id: String,
	pub alias: Option<Alias>,
	pub created_at: String,
	pub updated_at: String,
	pub interactions: Option<PlayerInteractions>,
	pub stats: PlayerStats,
}