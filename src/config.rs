use config::Config;
use std::collections::HashMap;

pub struct SettingsVariables {
    ignored_players: Option<Vec<String>>,
}

const SETTINGS: SettingsVariables = SettingsVariables{ignored_players: None};

pub fn create_config() {
    let settings = Config::builder()
        .add_source(config::File::with_name("overlay"))
        .build()
        .unwrap();

    println!("{:?}", settings.try_deserialize::<HashMap<String, String>>().unwrap());
}
