use crate::config::{Block, Config, ConfigItem};

impl Apache2Config for Block {

	fn to_apache(&self) -> String {
		self.to_string_level(0)
	}

	fn to_string_level(&self, level: usize) -> String {
		let mut s = String::new();
		let padding = " ".repeat(4 * level);
		let params = if self.params.len() != 0 {format!(" {}", self.params.join(" "))} else {String::new()};
		s.push_str(&format!("{}<{}{}>\n", padding, self.name, params));
		s.push_str(&self.config.to_string_level(level + 1));
		s.push_str(&format!("{}</{}>\n", padding, self.name));
		return s;
	}

}

impl Apache2Config for Config {

	fn to_apache(&self) -> String {
		self.to_string_level(0)
	}

	fn to_string_level(&self, level: usize) -> String {
		let mut s = String::new();
		let padding = " ".repeat(4 * level);
		for item in &self.content {
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

pub trait Apache2Config {
	fn to_string_level(&self, level: usize) -> String;
	fn to_apache(&self) -> String;
}
