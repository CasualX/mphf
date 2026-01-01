// Compile-time test: this const ensures the StaticMap is built at compile time
static_map! {
	TEST_MAP: i32, 2; {
		"apple" => 1,
		"banana" => 2,
		"cherry" => 3,
		"date" => 4,
		"elderberry" => 5,
	}
}

#[test]
fn lookup() {
	let map = TEST_MAP.as_ref();
	assert_eq!(map.get("apple"), Some(&1));
	assert_eq!(map.get("banana"), Some(&2));
	assert_eq!(map.get("cherry"), Some(&3));
	assert_eq!(map.get("date"), Some(&4));
	assert_eq!(map.get("elderberry"), Some(&5));
}

#[test]
fn missing_keys() {
	let map = TEST_MAP.as_ref();
	assert_eq!(map.get("fig"), None);
	assert_eq!(map.get("grape"), None);
	assert_eq!(map.get(""), None);
}

#[test]
fn larger() {
	#[derive(Copy, Clone, Debug, PartialEq)]
	enum Keyword { If, Else, While, For, Return, Fn, Let, Const, Struct, Enum }
	static_map!(KEYWORDS: Keyword, 16; {
		"if" => Keyword::If,
		"else" => Keyword::Else,
		"while" => Keyword::While,
		"for" => Keyword::For,
		"return" => Keyword::Return,
		"fn" => Keyword::Fn,
		"let" => Keyword::Let,
		"const" => Keyword::Const,
		"struct" => Keyword::Struct,
		"enum" => Keyword::Enum,
	});

	let keywords = KEYWORDS.as_ref();
	assert_eq!(keywords.get("if"), Some(&Keyword::If));
	assert_eq!(keywords.get("else"), Some(&Keyword::Else));
	assert_eq!(keywords.get("while"), Some(&Keyword::While));
	assert_eq!(keywords.get("for"), Some(&Keyword::For));
	assert_eq!(keywords.get("return"), Some(&Keyword::Return));
	assert_eq!(keywords.get("fn"), Some(&Keyword::Fn));
	assert_eq!(keywords.get("let"), Some(&Keyword::Let));
	assert_eq!(keywords.get("const"), Some(&Keyword::Const));
	assert_eq!(keywords.get("struct"), Some(&Keyword::Struct));
	assert_eq!(keywords.get("enum"), Some(&Keyword::Enum));
	assert_eq!(keywords.get("match"), None);
}
