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

}

pub struct Config {
	pub content: Vec<ConfigItem>
}

pub enum ConfigItem {
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

}

pub struct Block {
	pub name: String,
	pub params: Vec<String>,
	pub config: Config
}

pub struct Directive {
	pub name: String,
	pub values: Vec<String>
}
