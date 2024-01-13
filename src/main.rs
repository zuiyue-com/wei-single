extern crate wei_single;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    single();
    Ok(())
}

fn single() {
    let instance = wei_single_instance::SingleInstance::new("wei_single_instance").unwrap();
    if !instance.is_single() {
        println!("Another instance of this application is already running. Exiting...");
        return;
    }
    std::thread::sleep(std::time::Duration::from_secs(1000));
}