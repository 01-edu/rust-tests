// Implement the function bubble_sort which receives a vector Vec<i32>
// and return the same vector but in increasing order using the bubble
// sort algorithm

fn main() {
	let ref mut v = vec![3, 2, 4, 5, 1, 7];
	let mut b = v.clone();
	bubble_sort(v);
	println!("{:?}", v);

	b.sort();
	println!("{:?}", b);
}

fn bubble_sort(vec: &mut Vec<i32>) {
	let mut swapped = true;
	while swapped {
		swapped = false;
		for i in 1..vec.len() {
			if vec[i] < vec[i - 1] {
				let temp = vec[i - 1];
				vec[i - 1] = vec[i];
				vec[i] = temp;
				swapped = true;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ordering() {
		let ref mut v = vec![3, 2, 4, 5, 1, 7];
		let mut b = v.clone();

		b.sort();
		bubble_sort(v);
		assert_eq!(*v, b);
	}
}
