use async_std::task;
use csv::ReaderBuilder;
use serde::Deserialize;
use verb::{Kind, Verb};

#[cfg(test)]
mod tests {
	use crate::fetch_verbs;

	#[test]
	fn fetch() {
		let vec = fetch_verbs();
		assert_eq!(140, vec.len())
	}
}


pub fn fetch_verbs() -> Vec<Verb> {
	let string = fetch().unwrap();
	let records = parse(&string);
	let verbs = records.iter().map(|it| it.into()).collect::<Vec<Verb>>();
	verbs
}

fn fetch() -> Result<String, surf::Exception> {
	task::block_on(async {
		surf::get(URL).recv_string().await
	})
}

const URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vTfBMkjMiu7_64wQ8kEjv_PDWidfCWgK49SedQP72-1gbVxVqYiyrHlpkUAZEL-nkk5yKlbgB0tAstT/pub?gid=522266416&single=true&output=csv";

fn parse(s: &str) -> Vec<Record> {
	let mut reader = ReaderBuilder::new().from_reader(s.as_bytes());
	let records = reader.deserialize().map(|it| {
		let record: Record = it.unwrap();
		record
	}).collect::<Vec<_>>();
	records
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
	ch: u32,
	verb_type: String,
	meaning: String,
	reading: String,
	search: String,
}

impl From<&Record> for Verb {
	fn from(record: &Record) -> Self {
		Verb {
			ch: record.ch,
			kind: kind(&record.verb_type),
			search: record.search.to_owned(),
			english: record.meaning.to_owned(),
		}
	}
}

fn kind(str: &str) -> Kind {
	match str {
		"u" => Kind::U,
		"ru" => Kind::Ru,
		"suru" => Kind::Suru,
		"kuru" => Kind::Kuru,
		_ => panic!("Invalid kind: {}", str),
	}
}
