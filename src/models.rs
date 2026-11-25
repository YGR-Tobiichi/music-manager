use std::fmt;

// Enum for the status of a song.
#[derive(Debug, Clone, PartialEq)]
pub enum SongStatus {
    Playing,
    InQueue,
}

// Display implementation for SongStatus.
impl fmt::Display for SongStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SongStatus::Playing => write!(f, "Playing"),
            SongStatus::InQueue => write!(f, "In Queue"),
        }
    }
}

#[derive(Debug, Clone)] // Struct representing a song.
pub struct Song {
    pub name: String,
    pub duration: String,
    pub artist: String,
    pub status: SongStatus,
}

// Constructor for the Song struct.
impl Song {
    pub fn new(name: &str, duration: &str, artist: &str) -> Self {
        Self {
            name: name.to_string(),
            duration: duration.to_string(),
            artist: artist.to_string(),
            status: SongStatus::InQueue,
        }
    }
}

// Trait for displaying song details.
pub trait SongDetails {
    fn show_details(&self);
}

// Implementation of the SongDetails trait for the Song struct.
impl SongDetails for Song {
    fn show_details(&self) {
        println!("\n================ SONG DETAILS ================");
        println!("Title    : {}", self.name);
        println!("Artist   : {}", self.artist);
        println!("Duration : {}", self.duration);
        println!("Status   : {}", self.status);
        println!("==============================================\n");
    }
}

// Returns a vector of available songs for the program.
pub fn get_available_songs() -> Vec<Song> {
    vec![
        Song::new("World is Mine", "04:13", "Hatsune Miku"),
        Song::new("Ghost Rule", "03:27", "Hatsune Miku"),
        Song::new("Senbonzakura", "04:05", "Hatsune Miku"),
        Song::new("Rolling Girl", "03:17", "Hatsune Miku"),
        Song::new("Tell Your World", "04:17", "Hatsune Miku"),
        Song::new("Teto Territory", "03:24", "Kasane Teto"),
        Song::new("Override", "02:19", "Kasane Teto"),
        Song::new("Mesmerizer", "02:36", "Hatsune Miku & Kasane Teto"),
        Song::new("Fukireta (Mischievous Function)", "03:06", "Kasane Teto"),
        Song::new("Machine Love", "03:49", "Kasane Teto"),
        Song::new("ECHO", "03:52", "GUMI"),
        Song::new("KING", "02:14", "GUMI"),
        Song::new("Megumi Megumi Fire Endless Night", "03:54", "GUMI"),
        Song::new("Ama no Jaku", "03:02", "GUMI"),
        Song::new("Matryoshka", "03:21", "Hatsune Miku & GUMI"),
    ]
}
