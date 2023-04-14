use restson::*;
use super::types::*;

impl RestPath<()> for Rumpus<DelegationKeyThis> {
	fn get_path(_:()) -> Result<String, Error> {
		Ok(String::from("delegation/keys/@this"))
	}
}

//Not the full capabilities of this endpoint
impl RestPath<&str> for Rumpus<Vec<Player>> {
	fn get_path(user_code: &str) -> Result<String, Error> {
		Ok(format!("levelhead/players?userIds={}", user_code))
	}
}