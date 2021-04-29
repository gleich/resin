mod conf;

fn main() {
	let config = conf::read().expect("Failed to read from configuration file");
	println!("scopes:   {:?}", config.scopes);
	println!("brackets: {:?}", config.brackets)
}
