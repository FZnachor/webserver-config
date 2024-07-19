impl Config {

	pub fn new() -> Config {
		Config {content: vec![]}
	}

	pub fn add_directive(self: &mut Config, directive: Directive) {
		self.content.push(directive);
	}

	pub fn add(self: &mut Config, name: &str, value: &str) {
		self.content.push(Directive {name: name.to_string(), values: vec![value.to_string()]});
	}

	pub fn add_multi(self: &mut Config, name: &str, value: Vec<&str>) {
		let values: Vec<String> = value.iter().map(|&s| s.to_string()).collect();
		self.content.push(Directive {name: name.to_string(), values});
	}

	pub fn to_string(self: Config) -> String {
		self.to_string_level(0)
	}

	pub fn to_string_level(self: Config, level: usize) -> String {
		let mut s = String::new();
		let padding = " ".repeat(4 * level);
		for directive in self.content {
			s.push_str(&format!("{}{} {}\n", padding, directive.name, directive.values.join(" ")));
		}
		return s;
	}

}

pub struct Config {
	content: Vec<Directive>
}

impl Block {

	pub fn from(name: &str, params: Vec<&str>, conf: Config) -> Block {
		let params: Vec<String> = params.iter().map(|&s| s.to_string()).collect();
		Block {name: name.to_string(), params, config: conf}
	}

	pub fn to_string(self: Block) -> String {
		self.to_string_level(0)
	}

	pub fn to_string_level(self: Block, level: usize) -> String {
		let mut s = String::new();
		let padding = " ".repeat(4 * level);
		let params = if self.params.len() != 0 {format!(" {}", self.params.join(" "))} else {String::new()};
		s.push_str(&format!("{}<{}{}>\n", padding, self.name, params));
		s.push_str(&self.config.to_string_level(level + 1));
		s.push_str(&format!("{}</{}>\n", padding, self.name));
		return s;
	}

}

pub struct Block {
	name: String,
	params: Vec<String>,
	config: Config
}

pub struct Directive {
	name: String,
	values: Vec<String>
}
