pub struct Config {
	pub content: Vec<ConfigItem>
}

pub enum ConfigItem {
	Directive(Directive),
	Block(Block)
}

impl Config {

	pub fn new() -> Self {
		Self {content: vec![]}
	}

	pub fn add_directive(&mut self, directive: Directive) {
		self.content.push(ConfigItem::Directive(directive));
	}

	pub fn add_block(&mut self, block: Block) {
		self.content.push(ConfigItem::Block(block));
	}

	pub fn add(&mut self, name: &str, value: &str) {
		self.add_directive(Directive {
			name: name.to_string(),
			values: vec![value.to_string()]
		});
	}

	pub fn add_multi<T: Into<String>>(&mut self, name: &str, values: Vec<T>) {
		self.add_directive(Directive {
			name: name.to_string(),
			values: values.into_iter().map(Into::into).collect()
		});
	}

}

pub struct Block {
	pub name: String,
	pub params: Vec<String>,
	pub config: Config
}

impl Block {

	pub fn new(name: &str, param: &str) -> Self {
		Self::new_multi(name, vec![param])
	}

	pub fn new_multi<T: Into<String>>(name: &str, params: Vec<T>) -> Self {
		let conf = Config::new();
		Self::from(name, params, conf)
	}

	pub fn new_simple(name: &str) -> Self {
		Self::new_multi(name, Vec::<String>::new())
	}

	pub fn from<T: Into<String>>(name: &str, params: Vec<T>, conf: Config) -> Self {
		Self {
			name: name.to_string(),
			params: params.into_iter().map(Into::into).collect(),
			config: conf
		}
	}

	pub fn modify_config<F>(&mut self, mut f: F) where F: FnMut(&mut Config) {
		f(&mut self.config)
	}

}

pub struct Directive {
	pub name: String,
	pub values: Vec<String>
}
