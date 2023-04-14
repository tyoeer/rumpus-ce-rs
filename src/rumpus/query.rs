use std::fmt;
use super::types::Stat;

//Can't generate the struct with the macro because we want to include docs
#[derive(Default)]
pub struct PlayerSearch {
	///The parameter you want to sort on.
	///By default it returns results from largest to smallest: to inverse this just prefix with a -.
	///Eg. sort=Subscribers vs. sort=-Subscribers.
	sort: Option<String>,
	///Maximum number of results to return. There is a hard limit of 64 (subject to change) – you’ll have to page to obtain additional results.
	limit: Option<u8>,
	///Up to 16 (subject to change) comma-separated userIds. If set, only Levels created by the users in this list will be returned.
	user_ids: Option<String>,
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
		$callback!(sort, String, "sort");
		$callback!(limit, u8, "limit");
		$callback!(user_ids, String, "userIds");
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
macro_rules! format_parameter {
	($field:ident, $type:ty, $queryField:literal $(, last)?) => {
		pub fn $field<V: Into<$type>>(mut self, value: V) -> Self {
			self.$field = Some(value.into());
			self
		}
	};
}

impl PlayerSearch {
	pub fn new() -> Self {
		Self::default()
	}
	
	player_search_parameters!(format_parameter);
}

impl fmt::Display for PlayerSearch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut prev = false;
		
		macro_rules! format_parameter {
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
		
		player_search_parameters!(format_parameter);
		
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]	
	fn simple_query_string_test() {
		let q = PlayerSearch::new()
			.limit(13)
			.include_aliases(false);
		
		assert_eq!(format!("{}",q),"limit=13&includeAliases=false")
	}
}