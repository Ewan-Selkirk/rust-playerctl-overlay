use std::{
	process::Command,
	fmt
};

pub struct Player {
    name: String,
    metadata: SongMetadata,
    active: bool
}

pub struct SongMetadata {
    title: String,
    artist: String,
    album: String,
    art_url: String,
    length: String,
}

impl Player {
    fn new(player_name: String, is_active: bool) -> Player {
        Player {
            name: player_name,
            metadata: SongMetadata::new(),
            active: is_active
        }
    }
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Player: {} [Active: {}], Metadata: {}", self.name, self.active, self.metadata)
	}
}

impl SongMetadata {
    fn new() -> SongMetadata {
        SongMetadata {
            title: String::from("SONG TITLE"),
            artist: String::from("SONG ARTIST"),
            album: String::from("ALBUM"),
            art_url: String::from("https://img.freepik.com/premium-photo/isolated-skeleton-standing-alone-against-white-background_977505-204.jpg"),
            length: String::from("137840000")
        }
    }
}

impl fmt::Display for SongMetadata {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Title: {}, Artist: {}, Album: {}", self.title, self.artist, self.album
		)
	}
}

pub fn check_players() -> Option<Vec<Player>> {
    let players = Command::new("playerctl")
        .args(vec!["-l"])
        .output()
        .unwrap_or_else(|_| panic!())
        .stdout;

    let mut players_string = String::from_utf8(players).expect("Expected a UTF-8 String");
	players_string.pop();

    match players_string.as_str() {
        "No players found" => None,
        _ => {
            let p_names: Vec<&str> = players_string.split("\n").collect::<Vec<&str>>();
			let mut players: Vec<Player> = Vec::with_capacity(p_names.len());

			for (i, n) in p_names.iter().enumerate() {
				let p = Player::new(n.to_string(), matches!(i, 0));

				println!("{}", p);
				players.push(p);
			}

            Some(players)
        },
    }
}

pub fn call_playerctl(arg: &str, extra: Option<&str>) -> String {
    // Allow the use of playerctl functions such as 'duration' and 'uc'
    let form = match extra {
        None => format!("{{{{ {arg} }}}}"),
        Some(n) => format!("{{{{ {n}({arg}) }}}}")
    };

    // Call playerctl command
    let command = Command::new("playerctl")
        .args(vec!["metadata", arg, "-p", "spotify,%any", "--format", &form])
        .output()
        .unwrap_or_else(|_| panic!("Failed to run playerctl command"))
        .stdout;

    // Convert the command output to a String and remove the trailing newline.
    let mut value = String::from_utf8(command).expect("Failed to unwrap result from UTF-8");
    value.pop();

    value
}
