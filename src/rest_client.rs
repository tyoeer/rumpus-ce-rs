use restson::{RestClient, Error};

const KEY_HEADER: &str = "Rumpus-Delegation-Key";

/**

Creates a new [RestClient] with a given delegation key and url/endpoint.

# Errors
- If `key` is not a valid HTTP header
- If `url` is not a valid URL
*/
pub fn try_with_key_url(key: impl AsRef<str>, url: impl AsRef<str>) -> Result<RestClient, Error> {
	let mut client = RestClient::new(url.as_ref())?;
	client.set_header(KEY_HEADER, key.as_ref())?;
	Ok(client)
}

/**

Creates a new [RestClient] with a given delegation key and using [the default API url/endpoint](super::API_URL).

# Errors
If `key` is not a valid HTTP header.
*/
pub fn try_with_key(key: impl AsRef<str>) -> Result<RestClient, Error> {
	try_with_key_url(key, super::API_URL)
}

/**

Creates a new [RestClient] with a given delegation key and url/endpoint.

Panicking version of [try_with_key_url].

# Panics
- If `key` is not a valid HTTP header
- If `url` is not a valid URL
*/
pub fn with_key_url(key: impl AsRef<str>, url: impl AsRef<str>) -> RestClient {
	match try_with_key_url(key,url) {
		Result::Ok(c) => c,
		Result::Err(restson::Error::UrlError) => panic!("hardcoded bscotch URL isn't a valid URL"),
		Result::Err(restson::Error::InvalidValue) => panic!("api key is not a valid HTTP header"),
		Result::Err(_) => unreachable!(),
	}
}

/**

Creates a new [RestClient] with a given delegation key and using [the default API url/endpoint](super::API_URL).

Panicking version of [try_with_key].

# Panics
If `key` is not a valid HTTP header.
*/
pub fn with_key(key: impl AsRef<str>) -> RestClient {
	with_key_url(key, super::API_URL)
}