use std::fmt;
use thiserror::Error;
use super::types::Stat;

#[derive(Error, Debug)]
#[error("value/amount of items of {value} is larger than maximum {maximum}")]
pub struct LimitError {
	pub value: usize,
	pub maximum: usize
}

impl LimitError {
	fn new(value: usize, maximum: usize) -> Self {
		Self {
			value,
			maximum,
		}
	}
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sort<P: fmt::Display> {
	ascending: bool,
	property: P,
}

impl<P: fmt::Display> Sort<P> {
	pub fn new(property: P, ascending: bool) -> Self {
		Self {
			property,
			ascending,
		}
	}
}
impl<P: fmt::Display> fmt::Display for Sort<P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.ascending {
			write!(f, "-")?;
		}
		
		write!(f, "{}", self.property)
	}
}

macro_rules! setter {
	($field:ident, $type:ty, $queryField:literal, customSetter $(, $_:tt)?) => {};
	($field:ident, $type:ty, $queryField:literal $(, $_:tt)? ) => {
		pub fn $field(mut self, $field: impl Into<$type>) -> Self {
			self.$field = Some($field.into());
			self
		}
	};
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerSortProperty {
	CreatedAt,
	UpdatedAt,
	Subscribers,
	PlayTime,
	Plays,
	Trophies,
	Shoes,
	Crowns,
	Published,
}

impl fmt::Display for PlayerSortProperty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use PlayerSortProperty::*;
		match self {
			CreatedAt => write!(f, "createdAt"),
			UpdatedAt => write!(f, "updatedAt"),
			Subscribers => write!(f, "Subscribers"),
			PlayTime => write!(f, "PlayTime"),
			Plays => write!(f, "Plays"),
			Trophies => write!(f, "Trophies"),
			Shoes => write!(f, "Shoes"),
			Crowns => write!(f, "Crowns"),
			Published => write!(f, "Published"),
		}
	}
}

pub type PlayerSearchSort = Sort<PlayerSortProperty>;

//Can't generate the struct with the macro because we want to include docs
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PlayerSearch {
	///The parameter you want to sort on. By default it returns results from largest to smallest.
	pub sort: Option<PlayerSearchSort>,
	///Maximum number of results to return. There is a hard limit of [Self::MAX_LIMIT] (subject to change) – you’ll have to page to obtain additional results.
	pub limit: Option<u8>,
	///Up to 16 (subject to change) comma-separated userIds. If set, only Levels created by the users in this list will be returned.
	pub user_ids: Option<Vec<String>>,
	///Limit results to those with at most this many subscribers.
	pub max_subscribers: Option<Stat>,
	///Limit results to those with at least this many subscribers.
	pub min_subscribers: Option<Stat>,
	///Limit results to those with at most this many seconds of playtime.
	pub max_play_time: Option<Stat>,
	///Limit results to those with at least this many seconds of playtime.
	pub min_play_time: Option<Stat>,
	///Return profiles created at or after this date. Must be parsable by Javascript `new Date()`.
	pub min_created_at: Option<String>,
	///Return profiles created at or before this date. Must be parsable by Javascript `new Date()`.
	pub max_created_at: Option<String>,
	///Return profiles updated at or after this date. Must be parsable by Javascript `new Date()`.
	pub min_updated_at: Option<String>,
	///Return profiles updated at or before this date. Must be parsable by Javascript `new Date()`.
	pub max_updated_at: Option<String>,
	///If true, will add the alias field to the profile.
	///This prevents the need for additional requests to find aliases, but you should only set this if you will be using/displaying all returned aliases!
	pub include_aliases: Option<bool>,
	///If true, information about your interactions with returned users (e.g. “following”) will be included in the response.
	pub include_my_interactions: Option<bool>,
	///If sorting based on a value that can contain ties, subsequent pages will contain repeated results on ties.
	///Results are secondarily sorted on the _id field: if you provide the _id of the last result from your prior search
	/// in this field you will be able to page results even when there are ties.
	pub tiebreaker_item_id: Option<String>,
}

macro_rules! player_search_parameters {
	($callback:ident) => {
		$callback!(sort, PlayerSearchSort, "sort", customSetter);
		$callback!(limit, u8, "limit", customSetter);
		$callback!(user_ids, Vec<String>, "userIds", customSetter);
		$callback!(max_subscribers, Stat, "maxSubscribers");
		$callback!(min_subscribers, Stat, "minSubscribers");
		$callback!(max_play_time, Stat, "maxPlayTime");
		$callback!(min_play_time, Stat, "minPlayTime");
		$callback!(min_created_at, String, "minCreatedAt");
		$callback!(max_created_at, String, "maxCreatedAt");
		$callback!(min_updated_at, String, "minUpdatedAt");
		$callback!(max_updated_at, String, "maxUpdatedAt");
		$callback!(include_aliases, bool, "includeAliases");
		$callback!(include_my_interactions, bool, "includeMyInteractions");
		$callback!(tiebreaker_item_id, String, "tiebreakerItemId", last);
	}
}

impl PlayerSearch {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub const MAX_LIMIT: usize = 64;
	pub const MAX_USERS: usize = 16;
}

impl PlayerSearch {
	pub fn sort(mut self, property: PlayerSortProperty, ascending: bool) -> Self {
		self.sort = Some(PlayerSearchSort::new(property, ascending));
		self
	}
	
	///Maximum number of results to return. Returns an error if limit is higher than [Self::MAX_LIMIT]
	pub fn limit(mut self, limit: u8) -> Result<Self, LimitError> {
		if limit as usize > Self::MAX_LIMIT {
			Err(LimitError::new(limit as usize , Self::MAX_LIMIT))
		} else {
			self.limit = Some(limit);
			Ok(self)
		}
	}
	
	///Limit results to these user ids. Returns an error if the amount of users is higher than [Self::MAX_USERS]
	pub fn user_ids<S: Into<String>, V: Into<Vec<S>>>(mut self, user_ids: V) -> Result<Self, LimitError> {
		let user_ids = user_ids.into().into_iter().map(|s| s.into()).collect::<Vec<_>>();
		if user_ids.len() > Self::MAX_USERS {
			Err(LimitError::new(user_ids.len(), Self::MAX_USERS))
		}else {
			self.user_ids = Some(user_ids);
			Ok(self)
		}
	}
	
	player_search_parameters!(setter);
}

impl fmt::Display for PlayerSearch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut prev = false;
		
		//Can't put in outside scope because of macro hygiene and self & prev
		macro_rules! format_parameter {
			($field:ident, Vec<$type:ty>, $queryField:literal $(, $_:tt)?) => {};
			($field:ident, $_type:ty, $queryField:literal, last) => {
				if let Some(v) = &self.$field {
					if prev {
						write!(f, "&")?;
					}
					write!(f, "{}={}", $queryField,v)?;
					//prev = true;
				}
			};
			($field:ident, $_type:ty, $queryField:literal $(, $_:tt)?) => {
				if let Some(v) = &self.$field {
					if prev {
						write!(f, "&")?;
					}
					write!(f, "{}={}", $queryField,v)?;
					prev = true;
				}
			};
		}
		
		if let Some(v) = &self.user_ids {
			// if prev {
			// 	write!(f, "&")?;
			// }
			write!(f, "userIds=")?;
			for (i, code) in v.iter().enumerate() {
				write!(f, "{}{}", if i!=0 {","} else {""}, code)?;
			}
			prev = true;
		}
		
		player_search_parameters!(format_parameter);
		
		Ok(())
	}
}




#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelSortProperty {
	CreatedAt,
	PlayTime,
	ReplayValue,
	ExposureBucks,
	///Synonym of “ExposureBucks”
	QAScore,
	///Synonymous with the “Featured” list in-game. Is the default and can be used for Tower levels.
	HiddenGem,
}

impl fmt::Display for LevelSortProperty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use LevelSortProperty::*;
		match self {
			CreatedAt => write!(f, "createdAt"),
			PlayTime => write!(f, "PlayTime"),
			ReplayValue => write!(f, "ReplayValue"),
			ExposureBucks => write!(f, "ExposureBucks"),
			QAScore => write!(f, "QAScore"),
			HiddenGem => write!(f, "HiddenGem"),
		}
	}
}

pub type LevelSearchSort = Sort<LevelSortProperty>;

//Can't generate the struct with the macro because we want to include docs
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct LevelSearch {
	///The parameter you want to sort on. By default it returns results from largest to smallest.
	pub sort: Option<LevelSearchSort>,
	///Maximum number of results to return. There is a hard limit of [Self::MAX_LIMIT] (subject to change).
	pub limit: Option<u8>,
	///Up to [Self::MAX_USERS] user ids. If set, only Levels created by the users in this list will be returned. Sorted by createdAt.
	pub user_ids: Option<Vec<String>>,
	///Up to [Self::MAX_LEVELS] level ids (a.k.a. “share codes”). If set, only levels listed here will be returned. Sorted by createdAt.
	pub level_ids: Option<Vec<String>>,
	///A tagId that Levels must have. Levels can only have three tags. This must be a tagId, not the human-friendly, localized tag text!
	pub tags: Option<String>,
	///Only tower levels are searched by default unless the user_ids or level_ids parameters are set. In those cases only return Tower levels by setting this to true.
	pub tower: Option<bool>,
	///If true, only levels in the Marketing department are returned. By default only tower levels are returned.
	pub marketing: Option<bool>,
	///If true, only levels that were made for a dailyBuild are returned. Cannot be set in conjunction with marketing.
	pub daily_build: Option<bool>,
	///To save bandwidth, level stats are not returned by default. Only set to true if you need them!
	pub include_stats: Option<bool>,
	///To save bandwidth, leaderboards are not returned by default. Only set to true if you need them!
	pub include_records: Option<bool>,
	/**
	If true, will add the user field to the level and to all records.
	This prevents the need for additional requests to find aliases,
	but you should only set this if you will be using/displaying all returned aliases!
	*/
	pub include_aliases: Option<bool>,
	///Limit results to those levels accumulating at least this many seconds of playtime. Can only be used with Tower searches.
	pub min_play_time: Option<Stat>,
	///Limit results to those levels accumulating at most this many seconds of playtime.Can only be used with Tower searches.
	pub max_play_time: Option<Stat>,
	///Limit results to those levels accumulating at least this many Exposure Bucks.Can only be used with Marketing searches.
	pub min_exposure_bucks: Option<Stat>,
	///Limit results to those levels accumulating at most this many Exposure Bucks.Can only be used with Marketing searches.
	pub max_exposure_bucks: Option<Stat>,
	/**
	Limit results to those levels with at least this “ReplayValue”.
	Note that this value is based on an internal formula that is subject to frequent change,
	so use this field with care! Can only be used with Tower searches.
	*/
	pub min_replay_value: Option<Stat>,
	/**
	Limit results to those levels with at most this “ReplayValue”.
	Note that this value is based on an internal formula that is subject to frequent change,
	so use this field with care! Can only be used with Tower searches.
	*/
	pub max_replay_value: Option<Stat>,
	/**
	Limit results to those levels with at least this “HiddenGem”.
	Note that this value is based on an internal formula that is subject to frequent change,
	so use this field with care! Can only be used with Tower searches.
	*/
	pub min_hidden_gem: Option<Stat>,
	/**
	Limit results to those levels with at most this “HiddenGem”.
	Note that this value is based on an internal formula that is subject to frequent change,
	so use this field with care! Can only be used with Tower searches.
	*/
	pub max_hidden_gem: Option<Stat>,
	///Limit results to those with exactly this many diamonds (causes min/maxDiamonds to be ignored).
	pub diamonds: Option<u8>,
	/**
	Limit results to those levels with at least this many difficulty diamonds.
	Must be between 0 and 6. Can only be used with Tower searches.
	*/
	pub min_diamonds: Option<Stat>,
	/**
	Limit results to those levels with at most this many difficulty diamonds.
	Must be between 0 and 6, and must be greater than or equal to minDiamonds if also set. Can only be used with Tower searches.
	*/
	pub max_diamonds: Option<Stat>,
	///Limit results to those levels published no less than this many seconds ago.
	pub min_seconds_ago: Option<Stat>,
	///Limit results to those levels published no more than this many seconds ago.
	pub max_seconds_ago: Option<Stat>,
	/**
	If sorting based on a value that can contain ties, subsequent pages will contain repeated results on ties.
	Results are secondarily sorted on the _id/itemId field: if you provide the _id of the last result from your
	prior search in this field you will be able to page results even when there are ties.
	*/
	pub tiebreaker_item_id: Option<String>,
	///Return levels created at or after this date. Must be parsable by Javascript new Date(). Can be used with Tower and Daily Build searches.
	pub min_created_at: Option<String>,
	///Return levels created at or before this date. Must be parsable by Javascript new Date(). Can be used with Tower and Daily Build searches.
	pub max_created_at: Option<String>,
	///If true, information about your interactions with returned levels (any of bookmarked, favorited, liked, played, completed) will be included in the response.
	pub include_my_interactions: Option<bool>,
	///If true, levels that can only be played on beta clients will also be included. (“Beta” version is based on the most recent version marked as “released” in the Levelhead patchnotes.)
	pub include_beta: Option<bool>,
}

macro_rules! level_search_parameters {
	($callback:ident) => {
		$callback!(sort, LevelSearchSort, "sort", customSetter);
		$callback!(limit, u8, "limit", customSetter);
		$callback!(user_ids, Vec<String>, "userIds", customSetter);
		$callback!(level_ids, Vec<String>, "levelIds", customSetter);
		$callback!(tags, String, "tags");
		$callback!(tower, bool, "tower");
		$callback!(marketing, bool, "marketing");
		$callback!(daily_build, bool, "dailyBuild");
		$callback!(include_stats, bool, "includeStats");
		$callback!(include_records, bool, "includeRecords");
		$callback!(include_aliases, bool, "includeAliases");
		$callback!(min_play_time, Stat, "minPlayTime");
		$callback!(max_play_time, Stat, "maxPlayTime");
		$callback!(min_exposure_bucks, Stat, "minExposureBucks");
		$callback!(max_exposure_bucks, Stat, "maxExposureBucks");
		$callback!(min_replay_value, Stat, "minReplayValue");
		$callback!(max_replay_value, Stat, "maxReplayValue");
		$callback!(min_hidden_gem, Stat, "minHiddenGem");
		$callback!(max_hidden_gem, Stat, "maxHiddenGem");
		$callback!(diamonds, u8, "diamonds");
		$callback!(min_diamonds, Stat, "minDiamonds");
		$callback!(max_diamonds, Stat, "maxDiamonds");
		$callback!(min_seconds_ago, Stat, "minSecondsAgo");
		$callback!(max_seconds_ago, Stat, "maxSecondsAgo");
		$callback!(tiebreaker_item_id, String, "tiebreakerItemId");
		$callback!(min_created_at, String, "minCreatedAt");
		$callback!(max_created_at, String, "maxCreatedAt");
		$callback!(include_my_interactions, bool, "includeMyInteractions");
		$callback!(include_beta, bool, "includeBeta", last);
	}
}

impl LevelSearch {
	pub fn new() -> Self {
		Self::default()
	}
	
	pub const MAX_LIMIT: usize = 64;
	pub const MAX_USERS: usize = 16;
	pub const MAX_LEVELS: usize = 16;
}

impl LevelSearch {
	pub fn sort(mut self, property: LevelSortProperty, ascending: bool) -> Self {
		self.sort = Some(LevelSearchSort::new(property, ascending));
		self
	}
	
	///Maximum number of results to return. Returns an error if limit is higher than [Self::MAX_LIMIT]
	pub fn limit(mut self, limit: u8) -> Result<Self, LimitError> {
		if limit as usize > Self::MAX_LIMIT {
			Err(LimitError::new(limit as usize , Self::MAX_LIMIT))
		} else {
			self.limit = Some(limit);
			Ok(self)
		}
	}
	
	///Limit results to these user ids. Returns an error if the amount of users is higher than [Self::MAX_USERS]
	pub fn user_ids<S: Into<String>, V: Into<Vec<S>>>(mut self, user_ids: V) -> Result<Self, LimitError> {
		let user_ids = user_ids.into().into_iter().map(|s| s.into()).collect::<Vec<_>>();
		if user_ids.len() > Self::MAX_USERS {
			Err(LimitError::new(user_ids.len(), Self::MAX_USERS))
		}else {
			self.user_ids = Some(user_ids);
			Ok(self)
		}
	}
	
	///Limit results to these level ids. Returns an error if the amount of users is higher than [Self::MAX_LEVELS]
	pub fn level_ids<S: Into<String>, V: Into<Vec<S>>>(mut self, level_ids: V) -> Result<Self, LimitError> {
		let level_ids = level_ids.into().into_iter().map(|s| s.into()).collect::<Vec<_>>();
		if level_ids.len() > Self::MAX_LEVELS {
			Err(LimitError::new(level_ids.len(), Self::MAX_LEVELS))
		}else {
			self.level_ids = Some(level_ids);
			Ok(self)
		}
	}
	
	level_search_parameters!(setter);
}

impl fmt::Display for LevelSearch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut prev = false;
		
		if let Some(v) = &self.user_ids {
			// if prev {
			// 	write!(f, "&")?;
			// }
			write!(f, "userIds=")?;
			for (i, code) in v.iter().enumerate() {
				write!(f, "{}{}", if i!=0 {","} else {""}, code)?;
			}
			prev = true;
		}
		if let Some(v) = &self.level_ids {
			if prev {
				write!(f, "&")?;
			}
			write!(f, "levelIds=")?;
			for (i, code) in v.iter().enumerate() {
				write!(f, "{}{}", if i!=0 {","} else {""}, code)?;
			}
			prev = true;
		}
		
		//Can't put in outside scope because of macro hygiene and self & prev
		macro_rules! format_parameter {
			($field:ident, Vec<$type:ty>, $queryField:literal $(, $_:tt)?) => {};
			($field:ident, $_type:ty, $queryField:literal, last) => {
				if let Some(v) = &self.$field {
					if prev {
						write!(f, "&")?;
					}
					write!(f, "{}={}", $queryField,v)?;
					//prev = true;
				}
			};
			($field:ident, $_type:ty, $queryField:literal $(, $_:tt)?) => {
				if let Some(v) = &self.$field {
					if prev {
						write!(f, "&")?;
					}
					write!(f, "{}={}", $queryField,v)?;
					prev = true;
				}
			};
		}
		
		level_search_parameters!(format_parameter);
		
		Ok(())
	}
}




#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]	
	fn player_simple_query_string() -> Result<(), LimitError> {
		let q = PlayerSearch::new()
			.user_ids(vec!["test", "someone", "m7n6j8"])?
			.limit(13)?
			.include_aliases(false)
			.sort(PlayerSortProperty::CreatedAt, true);
		
		assert_eq!(format!("{}",q),"userIds=test,someone,m7n6j8&sort=-createdAt&limit=13&includeAliases=false");
		
		Ok(())
	}
	
	#[test]	
	fn level_simple_query_string() -> Result<(), LimitError> {
		let q = LevelSearch::new()
			.user_ids(vec!["test", "someone", "m7n6j8"])?
			.level_ids(vec!["best","epic"])?
			.limit(14)?
			.include_records(true)
			.sort(LevelSortProperty::PlayTime, false);

		assert_eq!(format!("{}",q),"userIds=test,someone,m7n6j8&levelIds=best,epic&sort=PlayTime&limit=14&includeRecords=true");

		Ok(())
	}
	
	#[test]
	fn limits_player_ok() {
		let x = PlayerSearch::new().user_ids(vec!["test"; PlayerSearch::MAX_USERS]);
		assert!(matches!(x, Result::Ok(_)));
		let x = PlayerSearch::new().limit(PlayerSearch::MAX_LIMIT as u8);
		assert!(matches!(x, Result::Ok(_)));
	}
	
	#[test]
	fn limits_player_err() {
		let x = PlayerSearch::new().user_ids(vec!["test"; PlayerSearch::MAX_USERS+1]);
		assert!(matches!(x, Result::Err(_)));
		let x = PlayerSearch::new().limit((PlayerSearch::MAX_LIMIT + 1) as u8);
		assert!(matches!(x, Result::Err(_)));
	}
	
	#[test]
	fn limits_level_ok() {
		let x = LevelSearch::new().user_ids(vec!["test"; LevelSearch::MAX_USERS]);
		assert!(matches!(x, Result::Ok(_)));
		let x = LevelSearch::new().level_ids(vec!["test"; LevelSearch::MAX_LEVELS]);
		assert!(matches!(x, Result::Ok(_)));
		let x = LevelSearch::new().limit(LevelSearch::MAX_LIMIT as u8);
		assert!(matches!(x, Result::Ok(_)));
	}
	
	#[test]
	fn limits_level_err() {
		let x = LevelSearch::new().user_ids(vec!["test"; LevelSearch::MAX_USERS+1]);
		assert!(matches!(x, Result::Err(_)));
		let x = LevelSearch::new().level_ids(vec!["test"; LevelSearch::MAX_LEVELS+1]);
		assert!(matches!(x, Result::Err(_)));
		let x = LevelSearch::new().limit((LevelSearch::MAX_LIMIT + 1) as u8);
		assert!(matches!(x, Result::Err(_)));
	}
}