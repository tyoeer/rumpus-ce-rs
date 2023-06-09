/*!

This crate tries to provide an interface to the [Rumpus CE API](https://www.bscotch.net/rumpus-ce) from Butterscotch Shenanigans.

It is build on [restson](https://crates.io/crates/restson), which is build on [hyper](https://crates.io/crates/hyper) and [tokio](https://crates.io/crates/tokio)

See the RestPath implementations for the [types::Rumpus#trait-implementations] type to see which endpoints you can currently use.

It provides some support for undocumented data through the `undocumented` feature, which is enabled by default.
It can be turned off in case Rumpus CE makes some changes to it's undocumented parts.

*/
#![allow(clippy::tabs_in_doc_comments)]

///Typed structs for Rumpus data
pub mod types;
///Stuff to handle parameters we send along
pub mod query;
///Handles integrating the right URL and header with the rets client
pub mod rest_client;

///Restson restPath implementations
mod endpoints;


//Restson requires the https:// for some reason

///The URL of the API
pub const API_URL: &str = "https://www.bscotch.net/api/";
///The URL of the beta API
pub const BETA_URL: &str = "https://beta.bscotch.net/api/";