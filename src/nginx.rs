use crate::config::{Block, Config, ConfigItem};

pub trait NginxConfig {
	fn to_string_level(&self, level: usize) -> String;
	fn to_nginx(&self) -> String;
}

impl NginxConfig for Block {

	fn to_nginx(&self) -> String {
		self.to_string_level(0)
	}

	fn to_string_level(&self, level: usize) -> String {
		let padding = " ".repeat(4 * level);
		let params = self.params.is_empty()
			.then(String::new)
			.unwrap_or_else(|| format!(" {}", self.params.join(" ")));

		let header = format!("{}{}{} {{\n", padding, self.name, params);
		let content = self.config.to_string_level(level + 1);
		let footer = format!("{}}}\n", padding);
		format!("{}{}{}", header, content, footer)
	}

}

impl NginxConfig for Config {

	fn to_nginx(&self) -> String {
		self.to_string_level(0)
	}

	fn to_string_level(&self, level: usize) -> String {
		let padding = " ".repeat(4 * level);
		self.content.iter().map(|item| {
			match item {
				ConfigItem::Directive(d) => {
					format!("{}{} {};\n", padding, d.name, d.values.join(" "))
				},
				ConfigItem::Block(b) => b.to_string_level(level)
			}
		}).collect()
	}

}
