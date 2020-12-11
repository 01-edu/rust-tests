// # Instructions:

// In this exercise you will be provided with a json file with data
// corresponding to git commits in github (extracted using the github
// rest api) your job is to extract the relevant data and place it in
// a struct called `CommitData` to get the following information:

// 1. Number of commits per author (identified by the github login)
// 2. And the number of commits per author

// Create two functions:
// fn commits_per_author(data: &Vec<CommitData>) -> HashMap<&str, u32>
// fn commits_per_date(data: &Vec<CommitData>) -> HashMap<String, u32>
// A week is represented by the a year followed by the number of the
// week for example January 1, 2020 is in week 1 of 2020 an will be
// represented by a String with the form "2020-W1"

// # Notions:
// https://docs.rs/chrono/0.4.19/chrono/#modules
// https://serde.rs/

use chrono::prelude::*;
use chrono::IsoWeek;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Week(IsoWeek);

use std::fmt;

impl fmt::Display for Week {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct Tree {
	sha: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
	login: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Commit {
	message: String,
	tree: Tree,
	author: AuthorInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthorInfo {
	name: String,
	date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommitData {
	sha: String,
	commit: Commit,
	author: Author,
}

use std::collections::HashMap;

fn commits_per_author<'a>(data: &'a Vec<CommitData>) -> HashMap<&'a str, u32> {
	let mut commits_per_author: HashMap<&'a str, u32> = HashMap::new();
	for commit in data {
		let count = commits_per_author.entry(&commit.author.login).or_insert(0);
		*count += 1;
	}
	println!("Total: {}", data.len());
	commits_per_author
}

fn commits_per_week(data: &Vec<CommitData>) -> HashMap<String, u32> {
	let mut commits_per_week: HashMap<String, u32> = HashMap::new();
	for commit in data {
		let count = commits_per_week
			.entry(Week(commit.commit.author.date.iso_week()).to_string())
			.or_insert(0);
		*count += 1;
	}
	commits_per_week
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	fn test_setup() -> Vec<CommitData> {
		let contents = fs::read_to_string("commits.json").unwrap();
		let serialized: Vec<CommitData> = serde_json::from_str(&contents).unwrap();
		serialized
	}

	#[test]
	fn test_commits_per_week() {
		let serialized = test_setup();
		let commits_per_week = commits_per_week(&serialized);
		println!("{:#?}", &commits_per_week);
		let date = [
			"2020-W47".to_string(),
			"2020-W43".to_string(),
			"2020-W36".to_string(),
			"2020-W50".to_string(),
			"2020-W40".to_string(),
			"2020-W44".to_string(),
			"2020-W46".to_string(),
			"2020-W31".to_string(),
			"2020-W45".to_string(),
			"2020-W49".to_string(),
		];

		let mut com_per_week = HashMap::new();
		let commits = [3, 1, 1, 2, 2, 5, 4, 1, 4, 7];

		for i in 0..date.len() {
			com_per_week.insert(date[i].clone(), commits[i].clone());
		}

		assert_eq!(com_per_week, commits_per_week);
	}

	#[test]
	fn test_commits_per_author() {
		let serialized = test_setup();
		let logins = [
			"RPigott",
			"RedSoxFan",
			"Xyene",
			"paul-ri",
			"JayceFayne",
			"mwenzkowski",
			"psnszsn",
			"emersion",
			"tamirzb",
			"ifreund",
			"homembaixinho",
		];
		let commits = [1, 1, 7, 2, 1, 3, 1, 10, 1, 1, 2];
		let mut expected = HashMap::new();

		for i in 0..logins.len() {
			expected.insert(logins[i], commits[i].to_owned());
		}

		let commits_per_author = commits_per_author(&serialized);
		println!("{:#?}", &commits_per_author);
		assert_eq!(expected, commits_per_author);
	}
}
