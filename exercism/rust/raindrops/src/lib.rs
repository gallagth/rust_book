pub fn raindrops(n: usize) -> String {
	let mut out = String::new();
	if n % 3 == 0 {
		out += "Pling";
	}
	if n % 5 == 0 {
		out += "Plang";
	}
	if n % 7 == 0 {
		out += "Plong";
	}
	match out.len() {
		0 => n.to_string(),
		_ => out
	}
}
