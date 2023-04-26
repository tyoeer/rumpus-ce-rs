use restson::*;
use super::types::*;
use super::query;

impl RestPath<()> for Rumpus<DelegationKeyInfo> {
	fn get_path(_:()) -> Result<String, Error> {
		Ok(String::from("delegation/keys/@this"))
	}
}

//Not the full capabilities of this endpoint
impl RestPath<query::PlayerSearch> for Rumpus<Vec<Player>> {
	fn get_path(query: query::PlayerSearch) -> Result<String, Error> {
		Ok(format!("levelhead/players?{}", query))
	}
}