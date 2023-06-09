An attempt at a Rust client for Rumpus CE.

## TODO:

- Set up a nice public API for fetching
- Make sure the Rumpus types are nice to work with (e.g. all the fields make sense and are documented)
	- We'll probably want wrapper types for level and player codes.
	- Figure out what types to use for the top-level Rumpus struct (e.g. figure out when the meta and errors field appear)
	- Alias .alias_type(/.context) and .anonymous appear to be mutually exclusive and can probably be combined into a single field
	- Handle datetimes
	- Different parameters of the LevelSearch query only work on certain search types (daily/tower/marketing).
	  It might be nice to split the query to represent that.
	- Better names:
		- ReplayValue = Spice
		- HiddenGem = Featured page in the tower
- Tests
- All endpoints
- Properly handle the rate limit
- Make sure everything that should derive things like Eq and Clone do so
- Add (optional?) support for the undocumented fields
- Use (optional?) caching

## Potentially useful links:

- [Official TypeScript SDK/Example  client](https://github.com/bscotch/rumpus-ce)
- [Rumpus CE landing page](https://www.bscotch.net/rumpus-ce)
- [Rumpus CE docs](https://www.bscotch.net/api/docs/community-edition/)
- [Rumpus CE Levelhead docs](https://www.bscotch.net/api/docs/levelhead/)
- [Level-Kit source](https://github.com/Radio-inactive/Levelhead-Web-Tools)
