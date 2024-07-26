# webserver-config

A Rust crate for generating configuration files for web servers. Supported web servers include Apache2 and nginx.

## Features

- **Very Simple**: Easy to use and integrate, with a small and manageable codebase.
- **Extensible**: Easily extendable to support additional web servers or configuration options.
- **No Dependencies**: Does not rely on external crates, ensuring lightweight and fast performance.

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
webserver-config = "0.1.0"
```

## Usage

Below are examples of using the crate to generate configurations for Apache2 and nginx web servers.

### Apache2 Config

Here's how you can generate a configuration for Apache2 web server:

```rs
let mut config = Config::new();

let mut block = Block::new("VirtualHost", "*:80");
block.modify_config(|c| {
	c.add("ServerName", "example.com");
	c.add_multi("ServerAlias", vec!["www.example.com", "example.net", "www.example.net"]);
	c.add("DocumentRoot", "/var/www/example");
});
config.add_block(block);

println!("{}", config.to_apache());
```

The code above will produce the following output:

```apacheconf
<VirtualHost *:80>
    ServerName example.com
    ServerAlias www.example.com example.net www.example.net
    DocumentRoot /var/www/example
</VirtualHost>
```

### nginx Config

Here's how you can generate a configuration for nginx web server:

```rs
let mut config = Config::new();

let mut block = Block::new_simple("server");
block.modify_config(|c| {
	c.add("listen", "80");
	c.add("listen", "[::]:80");
	c.add_multi("server_name", vec!["example.com", "www.example.com"]);
	c.add("root", "/var/www/example");
});
config.add_block(block);

println!("{}", config.to_nginx());
```

The code above will produce the following output:

```nginx
server {
    listen 80;
    listen [::]:80;
    server_name example.com www.example.com;
    root /var/www/example;
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
