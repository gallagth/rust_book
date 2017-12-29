extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
	let result = start.checked_add_signed(Duration::seconds(1_000_000_000));
	match result {
		Some(d) => d,
		None => panic!("Date overflow!"),
	}
}
