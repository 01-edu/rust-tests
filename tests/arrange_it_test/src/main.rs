// fn test_memory() {
// 	assert!(assert_no_alloc(|| is_empty("")));
// 	assert!(assert_no_alloc(|| is_ascii("rust")));
// 	assert!(assert_no_alloc(|| contains("rust", "ru")));
// 	assert!(("ru", "st") == assert_no_alloc(|| split_at("rust", 2)));
// 	assert!(1 == assert_no_alloc(|| find("rust", 'u')));
// }

#[allow(unused_imports)]
use arrange_it::*;
use assert_no_alloc::*;

#[global_allocator]
static A: AllocDisabler = AllocDisabler;

#[allow(dead_code)]
fn main() {
	println!("{:?}", arrange_phrase("is2 Thi1s T4est 3a"));
}

#[allow(dead_code)]
fn arrange_phrase_sol(phrase: &str) -> String {
	let nbrs: Vec<&str> = phrase.matches(char::is_numeric).collect();
	let a = &phrase.replace(char::is_numeric, "");
	let mut m: Vec<&str> = a.split_whitespace().collect();

	for (i, ele) in nbrs.iter().enumerate() {
		let strs: Vec<&str> = a.split_whitespace().collect();
		m[ele.parse::<usize>().unwrap() - 1] = strs[i];
	}
	m.join(" ")
}

#[test]
fn test_heap_memory_allocation() {
	let test_value = "w7ork t3he a4rt o5f Per1formance is2 a6voiding";
	assert_no_alloc(|| arrange_phrase_sol(test_value));
	let sol_violations = violation_count();
	assert_no_alloc(|| arrange_phrase(test_value));
	let stu_violations = violation_count() - sol_violations;
	
	assert!(
		stu_violations <= sol_violations,
		format!(
			"You are allocating to the heap {} time, and it must be less or equal to {} times",
			stu_violations, sol_violations
		)
	);
}

#[test]
fn test_function() {
	let cases = vec![
		"4of Fo1r pe6ople g3ood th5e the2",
		"is2 Thi1s T4est 3a",
		"w7ork t3he a4rt o5f Per1formance is2 a6voiding",
	];
	for v in cases {
		assert_eq!(arrange_phrase(v), arrange_phrase_sol(v));
	}
}
