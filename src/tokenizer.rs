use std::error::Error;

pub fn tokenizer(buffer: &String) -> Result<Vec<String>, Box<dyn Error>> {
/*  Mini tokenizer that's very hardcoded for this specific use.
	We do not handle newlines here because they should already be stripped out.
	If not for the single and double quote handling, we would get by with
		`buffer.split_ascii_whitespace().collect()`
*/

	let mut tokens = Vec::new();
	let buffer = buffer.trim().as_bytes();

	#[derive(PartialEq)]
	enum QuoteContext {
		Single,
		Double,
		None,
	}

	let mut quoted = QuoteContext::None;
	let mut i = 0;
	let mut word = String::new();

	while i < buffer.len() {
		match buffer[i] {
			b' ' => match quoted {
				QuoteContext::None => {
										tokens.push(word.clone());
										word.clear();

										//skip over adjacent spaces. This is important because we need to distinguish between a space, and an "".
										while i + 1 < buffer.len() && buffer[i + 1] == b' ' {
											i += 1;
										}
				},
				QuoteContext::Single | QuoteContext::Double => { word.push(buffer[i] as char); },
			},
			b'\'' => match quoted {
				QuoteContext::None => { quoted = QuoteContext::Single; },
				QuoteContext::Single => { quoted = QuoteContext::None; },
				QuoteContext::Double => { word.push(buffer[i] as char); },
			},
			b'"' => match quoted {
				QuoteContext::None => { quoted = QuoteContext::Double; },
				QuoteContext::Single => { word.push(buffer[i] as char); },
				QuoteContext::Double => { quoted = QuoteContext::None; },
			},
			_ => { word.push(buffer[i] as char); },
		}

		i += 1;

		if i == buffer.len() {
			tokens.push(word.clone()); //push the very last word into the tokens list
		}
	}

	if quoted != QuoteContext::None {
		return Err("error: mismatched quotes".into());
	}

	Ok(tokens)
}
