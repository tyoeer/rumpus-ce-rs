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
pub enum SortProperty {
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

impl fmt::Display for SortProperty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use SortProperty::*;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSearchSort {
	ascending: bool,
	property: SortProperty,
}

impl PlayerSearchSort {
	pub fn new(property: SortProperty, ascending: bool) -> Self {
		Self {
			property,
			ascending,
		}
	}
}
impl fmt::Display for PlayerSearchSort {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.ascending {
			write!(f, "-")?;
		}
		
		write!(f, "{}", self.property)
	}
}

//Can't generate the struct with the macro because we want to include docs
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PlayerSearch {
	///The parameter you want to sort on.
	///By default it returns results from largest to smallest: to inverse this just prefix with a -.
	///Eg. sort=Subscribers vs. sort=-Subscribers.
	sort: Option<PlayerSearchSort>,
	///Maximum number of results to return. There is a hard limit of 64 (subject to change) – you’ll have to page to obtain additional results.
	limit: Option<u8>,
	///Up to 16 (subject to change) comma-separated userIds. If set, only Levels created by the users in this list will be returned.
	user_ids: Option<Vec<String>>,
	///Limit results to those with at most this many subscribers.
	max_subscribers: Option<Stat>,
	///Limit results to those with at least this many subscribers.
	min_subscribers: Option<Stat>,
	///Limit results to those with at most this many seconds of playtime.
	max_play_time: Option<Stat>,
	///Limit results to those with at least this many seconds of playtime.
	min_play_time: Option<Stat>,
	///Return profiles created at or after this date. Must be parsable by Javascript `new Date()`.
	min_created_at: Option<Stat>,
	///Return profiles created at or before this date. Must be parsable by Javascript `new Date()`.
	max_created_at: Option<Stat>,
	///Return profiles updated at or after this date. Must be parsable by Javascript `new Date()`.
	min_updated_at: Option<Stat>,
	///Return profiles updated at or before this date. Must be parsable by Javascript `new Date()`.
	max_updated_at: Option<Stat>,
	///If true, will add the alias field to the profile.
	///This prevents the need for additional requests to find aliases, but you should only set this if you will be using/displaying all returned aliases!
	include_aliases: Option<bool>,
	///If true, information about your interactions with returned users (e.g. “following”) will be included in the response.
	include_my_interactions: Option<bool>,
	///If sorting based on a value that can contain ties, subsequent pages will contain repeated results on ties.
	///Results are secondarily sorted on the _id field: if you provide the _id of the last result from your prior search
	/// in this field you will be able to page results even when there are ties.
	tiebreaker_item_id: Option<String>,
}

macro_rules! player_search_parameters {
	($callback:ident) => {
		$callback!(sort, PlayerSearchSort, "sort");
		$callback!(limit, u8, "limit");
		$callback!(user_ids, Vec<String>, "userIds");
		$callback!(max_subscribers, Stat, "maxSubscribers");
		$callback!(min_subscribers, Stat, "minSubscribers");
		$callback!(max_play_time, Stat, "maxPlayTime");
		$callback!(min_play_time, Stat, "minPlayTime");
		$callback!(min_created_at, Stat, "minCreatedAt");
		$callback!(max_created_at, Stat, "maxCreatedAt");
		$callback!(min_updated_at, Stat, "minUpdatedAt");
		$callback!(max_updated_at, Stat, "maxUpdatedAt");
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

macro_rules! setter {
	(sort, $type:ty, $queryField:literal) => {};
	(limit, $type:ty, $queryField:literal) => {};
	(user_ids, $type:ty, $queryField:literal) => {};
	($field:ident, $type:ty, $queryField:literal $(, last)?) => {
		pub fn $field(mut self, $field: impl Into<$type>) -> Self {
			self.$field = Some($field.into());
			self
		}
	};
}
impl PlayerSearch {
	pub fn sort(mut self, property: SortProperty, ascending: bool) -> Self {
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
		
		macro_rules! format_parameter {
			(user_ids, $type:ty, $queryField:literal) => {};
			($field:ident, $_type:ty, $queryField:literal) => {
				if let Some(v) = &self.$field {
					if prev {
						write!(f, "&")?;
					}
					write!(f, "{}={}", $queryField,v)?;
					prev = true;
				}
			};
			($field:ident, $_type:ty, $queryField:literal, last) => {
				if let Some(v) = &self.$field {
					if prev {
						write!(f, "&")?;
					}
					write!(f, "{}={}", $queryField,v)?;
					//prev = true;
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

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]	
	fn simple_query_string_test() -> Result<(), LimitError> {
		let q = PlayerSearch::new()
			.user_ids(vec!["test", "someone", "m7n6j8"])?
			.limit(13)?
			.include_aliases(false)
			.sort(SortProperty::CreatedAt, true);
		
		assert_eq!(format!("{}",q),"userIds=test,someone,m7n6j8&sort=-createdAt&limit=13&includeAliases=false");
		
		Ok(())
	}
}