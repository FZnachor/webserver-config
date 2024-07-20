use std::borrow::BorrowMut;

impl Config {

	pub fn new() -> Config {
		Config {content: vec![]}
	}

	pub fn add_directive(self: &mut Config, directive: Directive) {
		self.content.push(ConfigItem::Directive(directive));
	}

	pub fn add_block(self: &mut Config, block: Block) {
		self.content.push(ConfigItem::Block(block));
	}

	pub fn add(self: &mut Config, name: &str, value: &str) {
		self.add_directive(Directive {name: name.to_string(), values: vec![value.to_string()]});
	}

	pub fn add_multi(self: &mut Config, name: &str, value: Vec<&str>) {
		let values: Vec<String> = value.iter().map(|&s| s.to_string()).collect();
		self.add_directive(Directive {name: name.to_string(), values});
	}

	pub fn to_string(self: Config) -> String {
		self.to_string_level(0)
	}

	pub fn to_string_level(self: Config, level: usize) -> String {
		let mut s = String::new();
		let padding = " ".repeat(4 * level);
		for item in self.content {
			match item {
				ConfigItem::Directive(d) => {
					s.push_str(&format!("{}{} {}\n", padding, d.name, d.values.join(" ")))
				},
				ConfigItem::Block(b) => {
					s.push_str(&b.to_string_level(level))
				}
			}
		}
		return s;
	}

}

pub struct Config {
	content: Vec<ConfigItem>
}

enum ConfigItem {
    Directive(Directive),
    Block(Block)
}

impl Block {

	pub fn new(name: &str, params: Vec<&str>) -> Block {
		let conf = Config::new();
		Block::from(name, params, conf)
	}

	pub fn from(name: &str, params: Vec<&str>, conf: Config) -> Block {
		let params: Vec<String> = params.iter().map(|&s| s.to_string()).collect();
		Block {name: name.to_string(), params, config: conf}
	}

    pub fn modify_config<F>(&mut self, mut f: F) where F: FnMut(&mut Config) {
        f(self.config.borrow_mut())
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
