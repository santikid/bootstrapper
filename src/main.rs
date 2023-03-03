use std::env;

mod bootstrap;
mod config;
mod features;
mod stow;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1);
    if command.is_none() {
        println!("no command given; options are: stow, unstow");
        return;
    }

    let config = config::Config::read_from_file("bootstrapper.json".into());
    if config.is_err() {
        println!(
            "could not read bootstrapper.json -> {}",
            config.err().unwrap()
        );
        return;
    }

    let config = config.unwrap();

    let want_features = features::Feature::want_features(&config.features);
    println!(
        "enabled features: {:?}",
        want_features
            .iter()
            .map(|f| format!("{} ({})", f.name, f.slug))
            .collect::<Vec<String>>()
    );

    match command {
        Some(c) if c == "stow" => {
            bootstrap::bootstrap(&config, false, &want_features);
        }
        Some(c) if c == "unstow" => {
            bootstrap::bootstrap(&config, true, &want_features);
        }
        _ => {
            println!(
                "unknown command: {}; options are: stow, unstow",
                command.unwrap()
            );
            return;
        }
    }
}
