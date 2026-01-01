#![feature(test)]

extern crate test;

// Test with many items of various lengths to get a good benchmark

mphf::static_map! {
	PERF_MAP: i32; {
		"alpha" => 1,
		"beta" => 2,
		"gamma" => 3,
		"delta" => 4,
		"epsilon" => 5,
		"zeta" => 6,
		"eta" => 7,
		"theta" => 8,
		"iota" => 9,
		"kappa" => 10,
		"lambda" => 11,
		"mu" => 12,
		"nu" => 13,
		"xi" => 14,
		"omicron" => 15,
		"pi" => 16,
		"rho" => 17,
		"sigma" => 18,
		"tau" => 19,
		"upsilon" => 20,
		"phi" => 21,
		"chi" => 22,
		"psi" => 23,
		"omega" => 24,
	}
}

#[bench]
fn bench_phf(b: &mut test::Bencher) {
	let map = test::black_box(PERF_MAP.as_ref());
	b.iter(|| {
		for key in map.keys() {
			test::black_box(map.get(key));
		}
	});
}

#[inline(never)]
fn match_key(key: &str) -> Option<i32> {
	let value = match key {
		"alpha" => 1,
		"beta" => 2,
		"gamma" => 3,
		"delta" => 4,
		"epsilon" => 5,
		"zeta" => 6,
		"eta" => 7,
		"theta" => 8,
		"iota" => 9,
		"kappa" => 10,
		"lambda" => 11,
		"mu" => 12,
		"nu" => 13,
		"xi" => 14,
		"omicron" => 15,
		"pi" => 16,
		"rho" => 17,
		"sigma" => 18,
		"tau" => 19,
		"upsilon" => 20,
		"phi" => 21,
		"chi" => 22,
		"psi" => 23,
		"omega" => 24,
		_ => return None,
	};
	Some(value)
}

#[bench]
fn bench_match(b: &mut test::Bencher) {
	let map = test::black_box(PERF_MAP.as_ref());
	let keys = map.keys();
	b.iter(|| {
		for &key in keys {
			test::black_box(match_key(key));
		}
	});
}
