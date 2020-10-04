/*!
Code generation for mphf.
*/

pub struct Options<'a> {
	pub name: &'a str,
	pub keys: &'a [&'a str],
	pub values: &'a [&'a str],
	pub seeds_len: usize,
	pub max_seed: u32,
	pub has_keys: bool,
	pub has_values: bool,
	pub has_index: bool,
	pub copy_values: bool,
}
impl<'a> Default for Options<'a> {
	fn default() -> Options<'a> {
		Options {
			name: "",
			keys: &[],
			values: &[],
			seeds_len: 0,
			max_seed: 0,
			has_keys: true,
			has_values: true,
			has_index: true,
			copy_values: true,
		}
	}
}

impl<'a> Options<'a> {
	/// Generates Rust source code.
	pub fn rust(&self) -> String {
		self::rust::generate(self)
	}
}

mod rust;
