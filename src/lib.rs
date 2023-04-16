/*!

This crate tries to provide an interface to the [Rumpus CE API](https://www.bscotch.net/rumpus-ce) from Butterscotch Shenanigans.

It is build on [restson](https://crates.io/crates/restson), which is build on [hyper](https://crates.io/crates/hyper) and [tokio](https://crates.io/crates/tokio)

See the RestPath implementations for the [types::Rumpus#trait-implementations] type to see which endpoints you can currently use.

*/

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