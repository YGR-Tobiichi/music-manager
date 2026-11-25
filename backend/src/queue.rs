use crate::models::{Song, SongDetails, SongStatus};

// Represents the music queue for the program.
pub struct MusicQueue {
    songs: Vec<Song>,
}

// Default constructor for a new MusicQueue.
impl MusicQueue {
    pub fn new() -> Self {
        Self { songs: Vec::new() }
    }

    /// Updates statuses based on queue position: index 0 is Playing, others are InQueue
    fn sync_statuses(&mut self) {
        for (index, song) in self.songs.iter_mut().enumerate() {
            if index == 0 {
                song.status = SongStatus::Playing;
            } else {
                song.status = SongStatus::InQueue;
            }
        }
    }

    // Adds a song to the queue, setting its status to Playing if the queue is empty, or InQueue otherwise.
    pub fn add_song(&mut self, mut song: Song) {
        song.status = if self.songs.is_empty() {
            SongStatus::Playing
        } else {
            SongStatus::InQueue
        };
        println!(
            "\n[+] Added '{}' by {} to the queue.",
            song.name, song.artist
        );
        self.songs.push(song);
        self.sync_statuses();
    }

    // Removes a song from the queue at the specified position.
    pub fn remove_song(&mut self, position: usize) -> Result<(), String> {
        if self.songs.is_empty() {
            return Err("The queue is currently empty.".to_string());
        }
        if position == 0 || position > self.songs.len() {
            return Err(format!(
                "Invalid position. Please choose a number between 1 and {}.",
                self.songs.len()
            ));
        }

        let removed = self.songs.remove(position - 1);
        self.sync_statuses();
        println!(
            "\n[-] Removed '{}' by {} from position #{}.",
            removed.name, removed.artist, position
        );
        Ok(())
    }

    // Moves a song to the front of the queue, setting its status to Playing.
    pub fn move_to_front(&mut self, position: usize) -> Result<(), String> {
        if self.songs.is_empty() {
            return Err("The queue is currently empty.".to_string());
        }
        if position == 0 || position > self.songs.len() {
            return Err(format!(
                "Invalid position. Please choose a number between 1 and {}.",
                self.songs.len()
            ));
        }
        if position == 1 {
            return Err("That song is already at position #1 (Playing).".to_string());
        }

        let song = self.songs.remove(position - 1);
        let song_name = song.name.clone();
        self.songs.insert(0, song);
        self.sync_statuses();

        println!("\n[^] Moved '{}' to position #1. Now Playing!", song_name);
        Ok(())
    }

    // Displays the current queue of songs.
    pub fn display_queue(&self) {
        println!("\n---------------- PLAYBACK QUEUE ----------------");
        if self.songs.is_empty() {
            println!("No songs in queue");
        } else {
            for (index, song) in self.songs.iter().enumerate() {
                println!(
                    "{}. [{}] {} - {} ({})",
                    index + 1,
                    song.status,
                    song.name,
                    song.artist,
                    song.duration
                );
            }
        }
        println!("------------------------------------------------");
    }

    // Shows the details of a song in the queue at the specified position.
    pub fn show_queue_song_details(&self, position: usize) -> Result<(), String> {
        if position == 0 || position > self.songs.len() {
            return Err(format!(
                "Invalid position. Please choose a number between 1 and {}.",
                self.songs.len()
            ));
        }
        self.songs[position - 1].show_details();
        Ok(())
    }

    // Returns whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }
}
