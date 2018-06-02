extern crate dire;

fn main() {
    let dirs = dirs::Directories::with_prefix("foobar", "Foobar").unwrap();
    println!("data_home:   {:?}", dirs.data_home());
    println!("config_home: {:?}", dirs.config_home());
    println!("cache_home:  {:?}", dirs.cache_home());
    println!("bin_home:    {:?}", dirs.bin_home());
}
