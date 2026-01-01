Minimal Perfect Hash Table
==========================

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/mphf.svg)](https://crates.io/crates/mphf)
[![docs.rs](https://docs.rs/mphf/badge.svg)](https://docs.rs/mphf)
[![Build status](https://github.com/CasualX/mphf/workflows/CI/badge.svg)](https://github.com/CasualX/mphf/actions)

An exercise in implementing minimal perfect hash table.

Examples
--------

```rust
mphf::static_map!(MY_TABLE: i32; {
	"apple" => 1,
	"banana" => 2,
	"cherry" => 3,
	"date" => 4,
});

let map = MY_TABLE.as_ref();
assert_eq!(map.get("banana"), Some(&2));
assert_eq!(map.get("fig"), None);
```

How it works
------------

There's an array/slice of keys and values of the same length. A hash function is used to map a key to an index in these arrays:

```rust
let index = hash(key) % N;
```

However, this does not guarantee that all the keys map to unique indices, leading to collisions.

To resolve this, the idea is to introduce a second level of hashing.

First, let's introduce a _keyed hash function_. A keyed hash function takes an additional seed parameter, effectively creating a family of hash functions where each seed selects a different one:

```rust
fn hash(seed: u32, key: &str) -> u32;
```

MurmurHash3 is exactly such a hash function and is used in this implementation.

In the first stage, we hash the key with seed `0`. We'll use this hash modulo the number of buckets to look up the 'displacement' seed for the second stage.

Then we hash the key again with the displacement seed to get the final index into the keys/values arrays:

```rust
static SEEDS: [u32; M] = [ ... ]; // Displacement seeds for each bucket
static KEYS: [&str; N] = [ ... ]; // Keys array

let bucket = hash(0, key) % SEEDS.len();
let seed = SEEDS[bucket];

let index = hash(seed, key) % KEYS.len();
if KEYS[index] != key {
	return None;
}

return Some(index);
```

The displacement seeds are chosen such that all keys map to unique indices in the second stage.

There is no clever trick being used here. When building the displacement seeds table we simply brute-force search for seeds that work for each bucket.

To make this work in practice, the entire construction runs in `const fn`, which imposes strict limits on how much brute-force search can be performed during compilation.

Conclusion
----------

Does it work? Yes. Is it faster than `match` on string literals? No. It's about 4x slower in my benchmarks.

```text
running 2 tests
test bench_match ... bench:          57.07 ns/iter (+/- 0.78)
test bench_phf   ... bench:         236.27 ns/iter (+/- 5.05)
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
