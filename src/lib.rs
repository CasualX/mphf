/*!
Minimally Perfect Hash Functions
================================


*/

#[cfg(feature = "codegen")]
pub mod codegen;

mod murmur3;
pub use self::murmur3::hash;

// Checks if the hashs with given seed are not already used and marks them as used.
fn check_seed(seed: u32, bucket: &[&str], used: &mut [bool]) -> bool {
	for &item in bucket {
		let h = hash(item.as_bytes(), seed) as usize % used.len();
		if used[h] {
			return false;
		}
		used[h] = true;
	}
	true
}

/// Builds the seeds table for a Minimally Perfect Hash Function over the input keys.
///
/// Returns `Err` if unable to bruteforce a seed which avoids hash collisions.
///
/// # Arguments
///
/// * `keys` is the list of static keys that will be used to build this hash table.
///
/// * `seeds_len` is the length of the intermediary list of seeds to avoid hash collisions.
///   A smaller number relative to the number of keys means that bruteforcing seeds may take significantly longer.
///   This value must be strictly greater than 0 or an `Err` is returned.
///
///   An interesting special case of `seeds_len = 1` this means there's a single hash function which is minimally perfect.
///   The returned seeds has a single value which can be passed directly to `hash(key, seed)`.
///   This only works for small sets of keys as the likelyhood of such a seed existing drops exponetially with the number of keys.
///
/// * `max_seed` is the cut-off for bruteforce searching of seeds which avoid hash collisions.
///   If the this number is reached the search stops and this function returns an `Err`.
///   Increasing the `seeds_len` has a much bigger impact than increasing `max_seed`.
///   In essence `max_seed` is used to avoid getting stuck looking for a perfect seed.
///
/// # Examples
///
/// ```
/// // The set of keys to build a minimally perfect hash function for
/// const KEYS: &[&str] = &["hello", "goodbye", "cat", "dog"];
///
/// // Build the mphf in two partitions
/// let seeds = mphf::build(KEYS, 2, 10000).unwrap();
/// println!("seeds: {:?}", seeds);
///
/// // Print the resulting hash values for each key
/// // Notice how each key maps to a unique number in range 0..KEYS.len()
/// for &key in KEYS {
/// 	let index = mphf::index(key, &seeds, KEYS.len()).unwrap();
/// 	println!("{}: {}", index, key);
/// }
/// ```
///
/// ```text
/// seeds: [0, 1]
/// 1: hello
/// 2: goodbye
/// 3: cat
/// 0: dog
/// ```
pub fn build(keys: &[&str], seeds_len: usize, max_seed: u32) -> Result<Vec<u32>, ()> {
	if seeds_len == 0 {
		return Err(());
	}

	// First pass over the input keys, bucket them by their hash
	let mut buckets = vec![(0usize, vec![]); seeds_len];
	for &key in keys {
		let h = hash(key.as_bytes(), 0) as usize % buckets.len();
		buckets[h].0 = h as usize;
		buckets[h].1.push(key);
	}

	// The table of seeds to disambiguate hash collisions
	let mut seeds = vec![u32::MAX; buckets.len()];

	// Caches used to detect hash collisions
	let mut used = vec![false; keys.len()];
	let mut tmp = vec![false; keys.len()];

	// Sort the buckets by the number of collisions
	// This will speed up bruteforcing a seed that breaks the collisions
	buckets.sort_unstable_by_key(|bucket| bucket.1.len());

	// Bruteforce a seed which avoids a hash collision with
	for &(index, ref bucket) in buckets.iter().rev() {
		if bucket.is_empty() {
			continue;
		}

		let mut seed = 0;
		while seed < max_seed {
			// Initialize the buffer for checking available seeds
			tmp.copy_from_slice(&used);
			if check_seed(seed, bucket, &mut tmp) {
				// Found a seed without hash collisions
				seeds[index] = seed;
				used.copy_from_slice(&tmp);
				break;
			}
			seed += 1;
		}
		if seed == max_seed {
			return Err(());
		}
	}

	return Ok(seeds);
}

/// Reorders the list of keys and values into their minimally perfect hash order.
pub fn reorder<T>(keys: &mut [&str], seeds: &[u32], mut values: Option<&mut [T]>) -> Option<()> {
	// If given the set of keys and values must have the same length
	if let Some(values) = &values {
		if keys.len() != values.len() {
			return None;
		}
	}
	// These have the same length so w/e is fine
	let values_len = keys.len();

	// Keep reordering until all keys and values have moved to the right position
	for i in 0..keys.len() {
		// Keep swapping the current element into the right position
		// This will swap w/e was in its position to our position
		// Repeat until we have the right element in our position
		loop {
			let j = index(keys[i], seeds, values_len)?;
			if i == j {
				break;
			}
			if let Some(values) = &mut values {
				values.swap(i, j);
			}
			keys.swap(i, j);
		}
	}

	Some(())
}

/// Returns the index of the given key in the mphf table.
#[inline]
pub fn index(key: &str, seeds: &[u32], values_len: usize) -> Option<usize> {
	let key = key.as_bytes();
	let h0 = hash(key, 0) as usize % seeds.len();
	let &seed = seeds.get(h0)?;
	if seed == u32::MAX {
		return None;
	}
	return Some(hash(key, seed) as usize % values_len);
}
/// Gets the value of the given key in the mphf table.
#[inline]
pub fn get<'a, T>(key: &str, seeds: &[u32], values: &'a [T]) -> Option<&'a T> {
	let index = index(key, seeds, values.len())?;
	values.get(index)
}
