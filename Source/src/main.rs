use std::env;
use std::fs;
use std::collections::HashMap;

fn main()
{
	let mut args = env::args();
	let in_file_path_maybe = args.nth(1);
	if in_file_path_maybe.is_none()
	{
		println!("No filename specified on command line!");
	}
	else
	{
		let in_file_path = in_file_path_maybe.unwrap();
		let file_contents_maybe = fs::read_to_string(in_file_path.clone());
		if file_contents_maybe.is_ok() == false
		{
			println!("Input file not found: {}", in_file_path);
		}
		else
		{
			let file_contents = file_contents_maybe.unwrap();
			println!("File contents:\n{}", file_contents);

			let counts_by_word =
				counts_by_word_gather_from_text(file_contents);

			let mut counts_by_word_sorted: Vec<(&String, &u32)> =
				counts_by_word.iter().collect();
			counts_by_word_sorted.sort_by
			(
				|a, b| b.1.cmp(a.1)
			);

			println!("Word count: {}", counts_by_word.len() );

			let words_and_counts_as_string =
				words_and_counts_to_string(counts_by_word_sorted);

			println!
			(
				"Words and counts: {}",
				words_and_counts_as_string
			);

			let out_file_path = "Out.txt";
			write_text_to_file_at_path
			(
				words_and_counts_as_string,
				out_file_path
			);
		}

	}

}

fn counts_by_word_gather_from_text(text: String) -> HashMap<String, u32>
{
	let words_as_slices: Vec<&str> =
		text.split_whitespace().collect();

	let mut counts_by_word: HashMap<String, u32> = HashMap::new();

	let chars_to_remove = &["!", ".", "?", "," ];

	for word_as_slice in &words_as_slices
	{
		let mut word = word_as_slice.to_string();
		word = word.to_lowercase();

		for char_to_remove in chars_to_remove
		{
			word = word.replace(char_to_remove, "");
		}

		if counts_by_word.contains_key(&word) == false
		{
			counts_by_word.insert(word.clone(), 0);
		}

		let count_for_word =
			*(counts_by_word.get(&word).unwrap()) + 1;
		counts_by_word.insert(word, count_for_word);
	}

	counts_by_word
}

fn words_and_counts_to_string(counts_by_word: Vec<(&String, &u32)>) -> String
{
	let mut words_and_counts_as_strings = vec![];
	words_and_counts_as_strings.push(String::from("\n"));

	for (word, count) in counts_by_word
	{
		let word_and_count_as_string =
			format!("{}: {}", word, count);
		words_and_counts_as_strings.push
		(
			word_and_count_as_string
		);
	}

	let words_and_counts_as_string =
		words_and_counts_as_strings.join("\n");

	words_and_counts_as_string
}

fn write_text_to_file_at_path(text_to_write: String, file_path: &str) -> ()
{
	let write_result =
		fs::write(file_path, text_to_write);

	if write_result.is_ok() == false
	{
		println!("Error writing output file!");
	}
	else
	{
		println!(
			"Output file {} written sucessfully.",
			file_path
		);
	}
}