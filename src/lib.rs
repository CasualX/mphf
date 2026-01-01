//! Minimal Perfect Hash Table.
//!
//! To get started, see the documentation for the [`static_map!`] macro.

#![no_std]

mod murmur3;

#[inline]
const fn hash(seed: u8, s: &str) -> u32 {
	murmur3::hash(s.as_bytes(), seed as u32)
}

#[inline]
const fn get_index(hash: u32, len: usize) -> usize {
	hash as usize % len
	// Optimized version to avoid division, but doesn't seem to increase performance in practice
	// let index = ((hash as u64 * len as u64) >> 32) as usize;
	// if index < len {
	// 	return index;
	// }
	// unsafe { std::hint::unreachable_unchecked() }
}

#[inline]
const fn str_eq(a: &str, b: &str) -> bool {
	let a_bytes = a.as_bytes();
	let b_bytes = b.as_bytes();
	if a_bytes.len() != b_bytes.len() {
		return false;
	}
	let mut i = 0;
	while i < a_bytes.len() {
		if a_bytes[i] != b_bytes[i] {
			return false;
		}
		i += 1;
	}
	true
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
pub struct StaticMapBuilder<T, const N: usize, const M: usize> {
	seeds: [u8; M],
	keys: [&'static str; N],
	values: [T; N],
}

impl<T: Copy, const N: usize, const M: usize> StaticMapBuilder<T, N, M> {
	/// Build a minimal perfect hash map from the given key-value pairs.
	/// Panics at compile time if a valid hash arrangement cannot be found.
	pub const fn build(keys: [&'static str; N], values: [T; N]) -> Self {
		assert!(N > 0, "Cannot build StaticMap with zero values");
		assert!(M > 0, "Bucket count M must be greater than zero");

		// u8::MAX means empty bucket (key not in map)
		// 0 means use original hash directly (no collision)
		// other values are disambiguation seeds
		let mut seeds = [u8::MAX; M];
		let mut placed = [false; N]; // tracks which output positions are occupied
		let mut output_keys: [&str; N] = [""; N];
		let mut output_vals: [T; N] = [values[0]; N]; // temporary init with first value

		// Count the number of hash collisions per bucket
		let mut bucket_sizes = [0usize; M];
		let mut i = 0;
		while i < N {
			let bucket = get_index(hash(0, keys[i]), M);
			bucket_sizes[bucket] += 1;
			i += 1;
		}

		// Process buckets from largest to smallest
		let mut processed_buckets = [false; M];
		let mut buckets_done = 0;

		while buckets_done < M {
			// Find the largest unprocessed bucket
			let mut max_size = 0;
			let mut max_bucket = 0;
			let mut b = 0;
			while b < M {
				if !processed_buckets[b] && bucket_sizes[b] >= max_size {
					max_size = bucket_sizes[b];
					max_bucket = b;
				}
				b += 1;
			}

			processed_buckets[max_bucket] = true;
			buckets_done += 1;

			if max_size == 0 {
				// Empty bucket, seed stays u8::MAX
				continue;
			}

			// Collect indices of keys in this bucket
			let mut bucket_key_indices: [usize; N] = [0; N];
			let mut bucket_count = 0;
			let mut k = 0;
			while k < N {
				if get_index(hash(0, keys[k]), M) == max_bucket {
					bucket_key_indices[bucket_count] = k;
					bucket_count += 1;
				}
				k += 1;
			}

			// Try seed=0 first (use original hash directly)
			// Then try other seeds for disambiguation
			let mut seed = 0u8;
			let found_seed = loop {
				let mut candidate_positions: [usize; N] = [0; N];
				let mut collision = false;

				// Check if this seed causes any collisions
				let mut j = 0;
				while j < bucket_count {
					let key_idx = bucket_key_indices[j];
					let pos = get_index(hash(seed, keys[key_idx]), N);

					// Check against already placed positions
					if placed[pos] {
						collision = true;
						break;
					}

					// Check against other positions in this batch
					let mut p = 0;
					while p < j {
						if candidate_positions[p] == pos {
							collision = true;
							break;
						}
						p += 1;
					}
					if collision {
						break;
					}

					candidate_positions[j] = pos;
					j += 1;
				}

				if !collision {
					// Found a valid seed! Place all keys from this bucket
					let mut j = 0;
					while j < bucket_count {
						let key_idx = bucket_key_indices[j];
						let pos = candidate_positions[j];
						placed[pos] = true;
						output_keys[pos] = keys[key_idx];
						output_vals[pos] = values[key_idx];
						j += 1;
					}
					break seed;
				}

				seed += 1;
				assert!(seed < u8::MAX, "Failed to find valid seed for StaticMap, try using more buckets");
			};

			seeds[max_bucket] = found_seed;
		}

		StaticMapBuilder {
			seeds,
			keys: output_keys,
			values: output_vals,
		}
	}
}
impl<T, const N: usize, const M: usize> StaticMapBuilder<T, N, M> {
	pub const fn as_ref(&'static self) -> StaticMap<T> {
		StaticMap {
			keys: StaticKeys {
				seeds: &self.seeds,
				keys: &self.keys,
			},
			values: &self.values,
		}
	}
}

/// Creates a minimal perfect hash map at compile time.
///
/// Arguments:
/// - `$vis`: Visibility modifier (e.g. `pub`, `pub(crate)`, or nothing for private)
/// - `$name`: Name of the static variable to create
/// - `$T`: Type of the values stored in the map
/// - `$M`: Number of buckets to use (default is half the number of entries)
/// - Key-value pairs in the form `{ "key1" => value1, "key2" => value2, ... }`
///
/// Example:
/// ```
/// mphf::static_map!(MY_TABLE: i32; {
/// 	"apple" => 1,
/// 	"banana" => 2,
/// 	"cherry" => 3,
/// 	"date" => 4,
/// });
///
/// let map = MY_TABLE.as_ref();
/// assert_eq!(map.get("banana"), Some(&2));
/// assert_eq!(map.get("fig"), None);
/// ```
#[macro_export]
macro_rules! static_map {
	($vis:vis $name:ident: $T:ty; { $( $key:expr => $val:expr ),* $(,)? }) => {
		$vis static $name: $crate::StaticMapBuilder<$T, {[$($key),*].len()}, {[$($key),*].len() / 2}> = $crate::StaticMapBuilder::build(
			[$($key),*],
			[$($val),*],
		);
	};
	($vis:vis $name:ident: $T:ty, $M:expr; { $( $key:expr => $val:expr ),* $(,)? }) => {
		$vis static $name: $crate::StaticMapBuilder<$T, {[$($key),*].len()}, $M> = $crate::StaticMapBuilder::build(
			[$($key),*],
			[$($val),*],
		);
	};
}

#[derive(Copy, Clone, Debug)]
struct StaticKeys {
	seeds: &'static [u8],
	keys: &'static [&'static str],
}

impl StaticKeys {
	#[inline(never)]
	fn get_index(&self, key: &str) -> usize {
		if self.seeds.len() == 0 || self.keys.len() == 0 {
			return usize::MAX;
		}

		let mut h = hash(0, key);
		let bucket = get_index(h, self.seeds.len());
		let seed = self.seeds[bucket];

		// Key definitely not in map
		if seed == u8::MAX {
			return usize::MAX;
		}

		// Use disambiguation seed if needed
		if seed != 0 {
			h = hash(seed, key);
		}
		let index = get_index(h, self.keys.len());

		// Check for key match
		if !str_eq(self.keys[index], key) {
			return usize::MAX;
		}

		return index;
	}
}

/// Minimal perfect hash map with string keys.
///
/// See the [`static_map!`] macro for how to create one.
#[derive(Copy, Clone, Debug)]
pub struct StaticMap<T: 'static> {
	keys: StaticKeys,
	values: &'static [T],
}

impl<T> StaticMap<T> {
	/// Returns the list of keys in the map.
	#[inline]
	pub fn keys(&self) -> &'static [&'static str] {
		self.keys.keys
	}
	/// Returns the list of values in the map.
	#[inline]
	pub fn values(&self) -> &'static [T] {
		self.values
	}
	/// Gets the value associated with the given key, or `None` if not found.
	#[inline]
	pub fn get(&self, key: &str) -> Option<&T> {
		let index = self.keys.get_index(key);
		self.values.get(index)
	}
}

#[cfg(test)]
mod tests;
