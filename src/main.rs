use sthin::configs::Configs;

fn main() {
    let config = Configs::get().expect("Failed to read configuration");
    println!("Config: {:?}", config);
}
