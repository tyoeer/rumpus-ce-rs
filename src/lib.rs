///Typed structs for Rumpus data
pub mod types;
///Stuff to handle parameters we send along
pub mod query;

///Restson restPath implementations
mod endpoints;


//Restson requires the https:// for some reason

///The URL of the API
pub const API_URL: &str = "https://www.bscotch.net/api/";
///The URL of the beta API
pub const BETA_URL: &str = "https://beta.bscotch.net/api/";