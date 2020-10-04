use super::Options;

pub fn generate(input: &Options) -> String {
	let seeds = crate::build(input.keys, input.seeds_len, input.max_seed).unwrap();
	let mut keys = input.keys.to_vec();
	let mut values = input.values.to_vec();
	crate::reorder(&mut keys, &seeds, Some(&mut values)).unwrap();

	format_xml::template!(
		"pub mod "{input.name}" {\n"
		"\tpub static SEEDS: [u32; "{seeds.len()}"] = [" for &seed in (&seeds) { {seed}"," } "];\n"
		"\tpub static KEYS: [&str; "{keys.len()}"] = [" for &key in (&keys) { "\""{key}"\"," } "];\n"
		"\tpub static VALUES: [&str; "{values.len()}"] = [" for &value in (&values) { "\""{value}"\"," } "];\n"
		if (input.has_keys) {
			"\t#[inline] pub fn key(key: &str) -> Option<&'static str> { ::mphf::get(key, &SEEDS, &VALUES).copied() }\n"
			"\t#[inline] pub fn keys() -> impl Iterator<Item = &'static str> { KEYS.iter().copied() }\n"
		}
		if (input.has_values) {
			if (input.copy_values) {
				"\t#[inline] pub fn value(key: &str) -> Option<&'static str> { ::mphf::get(key, &SEEDS, &VALUES).copied() }\n"
				"\t#[inline] pub fn values() -> impl Iterator<Item = &'static str> { VALUES.iter().copied() }\n"
			}
			else {
				"\t#[inline] pub fn value(key: &str) -> Option<&'static &'static str> { ::mphf::get(key, &SEEDS, &VALUES) }\n"
				"\t#[inline] pub fn values() -> impl Iterator<Item = &'static &'static str> { VALUES.iter() }\n"
			}
		}
		if (input.has_index) {
			"\t#[inline] pub fn index(key: &str) -> Option<usize> { ::mphf::index(key, &SEEDS, VALUES.len()) }\n"
		}
		if (input.has_keys && input.has_values) {
			"\t#[inline] pub fn iter() -> impl Iterator<Item = (&'static str, &'static str)> { (0.."{keys.len()}").map(|i| (KEYS[i], VALUES[i])) }\n"
		}
		"}\n"
	).to_string()
}
