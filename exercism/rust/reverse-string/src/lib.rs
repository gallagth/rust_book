pub fn reverse(input: &str) -> String {
	let chars = input.chars();
	let mut out = String::with_capacity(input.len());
	for c in chars.rev() {
		out.push(c);
	}
	out
}
