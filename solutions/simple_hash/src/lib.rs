// Create the function `contain` that checks a `HashMap` to see if it contains the given key.
// Create the function `remove` that removes a given key from the `HashMap`.

use std::collections::HashMap;

pub fn contain(h: &HashMap<&str, i32>, s: &str) -> bool {
	match h.get(s) {
		Some(_hash) => true,
		None => false,
	}
}

pub fn remove(h: &mut HashMap<&str, i32>, s: &str) {
	h.remove(s);
}

// fn main() {
//     let mut hash: HashMap<&str, i32> = HashMap::new();
//     hash.insert("Daniel", 122);
//     hash.insert("Ashley", 333);
//     hash.insert("Katie", 334);
//     hash.insert("Robert", 14);

//     println!(
//         "Does the HashMap contains the name Roman? => {}",
//         contain(hash.clone(), "Roman")
//     );
//     println!(
//         "Does the HashMap contains the name Katie? => {}",
//         contain(hash.clone(), "Katie")
//     );
//     println!("Removing Robert {:?}", remove(hash.clone(), "Robert"));
//     println!("Hash {:?}", hash);
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_contains() {
		let mut s = HashMap::new();

		s.insert("Pedro", 43);
		s.insert("Ralph", 12);
		s.insert("Johnny", 546);
		s.insert("Albert", 12323214);

		assert_eq!(true, contain(&s, "Pedro"));
		assert_eq!(true, contain(&s, "Ralph"));
		assert_eq!(true, contain(&s, "Johnny"));
		assert_eq!(true, contain(&s, "Albert"));
		assert_eq!(false, contain(&s, "Marco"));
		assert_eq!(false, contain(&s, "Joan"));
		assert_eq!(false, contain(&s, "Louise"));
	}

	#[test]
	fn test_remove() {
		let mut n = HashMap::new();
		n.insert("Dani Sordo", 37);
		n.insert("Sébastien Loeb", 46);
		n.insert("Ott Tanak", 32);
		n.insert("Thierry Neuville", 32);

		remove(&mut n, "Dani Sordo");
		assert_eq!(true, contain(&n, "Ott Tanak"));
		assert_eq!(false, contain(&n, "Dani Ŝordo"))
	}
}
