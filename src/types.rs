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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Record {
	user_id: String,
	alias: Alias,
	value: f32,
	created_at: String,
}

///Indicator how much objects of certain categories this level has
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct LevelContents {
	pub world: Stat,
	pub movement: Stat,
	pub puzzles: Stat,
	pub enemies: Stat,
	pub hazards: Stat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct LevelStats {
	#[serde(default)]
	pub attempts: Stat,
	#[serde(default)]
	pub favorites: Stat,
	#[serde(default)]
	pub likes: Stat,
	pub perk_points: Option<Stat>,
	#[serde(default)]
	pub play_time: Stat,
	#[serde(default)]
	pub players: Stat,
	#[serde(default)]
	pub replay_value: Stat,
	pub clear_rate: f32,
	///Difficulty diamonds. 6 for uncleared.
	//TODO: THis would probably be better as an enum
	pub diamonds: u8,
	#[serde(default)]
	pub successes: Stat,
	pub time_per_win: f32,
	#[serde(default)]
	pub exposure_bucks: Stat,
	pub failure_rate: f32,
	
	#[cfg(feature="undocumented")]
	pub hidden_gem: Stat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct LevelRecords {
	high_score: Vec<Record>,
	fastest_time: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LevelInteractions {
	pub bookmarked: bool,
	pub liked: bool,
	pub favorited: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Level {
	#[serde(rename="_id")]
	pub id: String,
	///Content version? Some notes are collected at <https://github.com/tyoeer/Chaoshead/issues/22>
	pub cv: Stat,
	pub level_id: String,
	pub user_id: String,
	pub alias: Option<Alias>,
	pub avatar_id: String,
	pub title: String,
	pub locale_id: Stat,
	pub locale: String,
	pub created_at: String,
	pub updated_at: String,
	pub tower: Option<bool>,
	pub daily_build: Option<bool>,
	pub tower_trial: bool,
	pub required_players: u8,
	///In seconds. Appears to have 2 significant decimals.
	pub creator_time: f32,
	pub game_version: Option<String>,
	pub tags: Vec<String>,
	pub tag_names: Vec<String>,
	pub content: LevelContents,
	pub stats: Option<LevelStats>,
	pub records: Option<LevelRecords>,
	pub interactions: Option<LevelInteractions>,
}