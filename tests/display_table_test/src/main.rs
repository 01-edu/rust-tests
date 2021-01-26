use display_table::Table;

fn main() {
	let mut table = Table::new();
	println!("{}", table);
	table.headers = vec![
		String::from("Model"),
		String::from("Piece N°"),
		String::from("In Stock"),
		String::from("Description"),
	];
	table.add_row(&[
		String::from("model 1"),
		String::from("43-EWQE304"),
		String::from("30"),
		String::from("Piece for x"),
	]);
	table.add_row(&[
		String::from("model 2"),
		String::from("98-QCVX5433"),
		String::from("100000000"),
		String::from("-"),
	]);
	table.add_row(&[
		String::from("model y"),
		String::from("78-NMNH"),
		String::from("60"),
		String::from("nothing"),
	]);
	println!("{}", table);
}

#[cfg(test)]
mod tests {
	use display_table::*;

	#[test]
	fn it_displays() {
		let mut table = Table::new();
		table.headers = vec![
			"Name".to_string(),
			"Last Name".to_string(),
			"ID Number".to_string(),
		];
		table.add_row(&[
			"Ackerley".to_string(),
			"Fillips".to_string(),
			"123456789".to_string(),
		]);
		table.add_row(&[
			"Adamaris".to_string(),
			"Fillips".to_string(),
			"1111123456789".to_string(),
		]);
		table.add_row(&[
			"Ackerley".to_string(),
			"Fillips".to_string(),
			"123456789".to_string(),
		]);
		assert_eq!(
			table.to_string(),
			"|   Name   | Last Name |   ID Number   |\n|----------+-----------+---------------|\n| Ackerley |  Fillips  |   123456789   |\n| Adamaris |  Fillips  | 1111123456789 |\n| Ackerley |  Fillips  |   123456789   |"
		);
	}

	// An empty table must not display anything
	#[test]
	fn display_empty() {
		let table = Table::new();
		assert_eq!(table.to_string(), "");
	}
}
