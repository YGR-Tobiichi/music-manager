mod models;
mod queue;

use models::{Song, SongDetails, get_available_songs};
use queue::MusicQueue;
use std::io::{self, Write};

// Receives a prompt string and returns an optional usize input from the user.
fn read_usize(prompt: &str) -> Option<usize> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        input.trim().parse::<usize>().ok()
    } else {
        None
    }
}

fn display_available_songs(catalog: &[Song]) {
    println!("\n=============== AVAILABLE SONGS ================");
    for (index, song) in catalog.iter().enumerate() {
        println!(
            "{:2}. {} - {} ({})",
            index + 1,
            song.name,
            song.artist,
            song.duration
        );
    }
    println!("================================================");
}

fn display_menu() {
    println!("\n==================== MENU ======================");
    println!("1. Add a song to the queue");
    println!("2. Remove a song from the queue (by position)");
    println!("3. Move a song to the front of the queue (Play now)");
    println!("4. View song details");
    println!("5. Show available songs");
    println!("6. Exit");
    println!("================================================");
}

fn main() {
    let available_songs = get_available_songs();
    let mut playback_queue = MusicQueue::new();

    println!("=== VOCALOID / UTAU MUSIC QUEUE MANAGER ===");

    loop {
        playback_queue.display_queue();
        display_menu();

        match read_usize("Select an option (1-6): ") {
            Some(1) => {
                match read_usize(&format!(
                    "Enter the song number to add (1-{}): ",
                    available_songs.len()
                )) {
                    Some(num) if num >= 1 && num <= available_songs.len() => {
                        let selected_song = available_songs[num - 1].clone();
                        playback_queue.add_song(selected_song);
                    }
                    _ => println!("\n[!] Invalid song number."),
                }
            }
            Some(2) => {
                if playback_queue.is_empty() {
                    println!("\n[!] Cannot remove: No songs in queue.");
                } else {
                    match read_usize("Enter the queue position to remove: ") {
                        Some(pos) => {
                            if let Err(err) = playback_queue.remove_song(pos) {
                                println!("\n[!] {}", err);
                            }
                        }
                        None => println!("\n[!] Please enter a valid position number."),
                    }
                }
            }
            Some(3) => {
                if playback_queue.is_empty() {
                    println!("\n[!] Cannot move: No songs in queue.");
                } else {
                    match read_usize("Enter the queue position to move to #1: ") {
                        Some(pos) => {
                            if let Err(err) = playback_queue.move_to_front(pos) {
                                println!("\n[!] {}", err);
                            }
                        }
                        None => println!("\n[!] Please enter a valid position number."),
                    }
                }
            }
            Some(4) => {
                println!("\nView details from:");
                println!("1. Available songs catalog");
                println!("2. Current playback queue");
                match read_usize("Choose source (1-2): ") {
                    Some(1) => {
                        match read_usize(&format!(
                            "Enter catalog song number (1-{}): ",
                            available_songs.len()
                        )) {
                            Some(num) if num >= 1 && num <= available_songs.len() => {
                                available_songs[num - 1].show_details();
                                println!("Press Enter to continue.");
                                io::stdout().flush().unwrap();

                                io::stdin().read_line(&mut String::new()).unwrap();
                            }
                            _ => println!("\n[!] Invalid song number."),
                        }
                    }
                    Some(2) => {
                        if playback_queue.is_empty() {
                            println!("\n[!] No songs in queue.");
                        } else {
                            match read_usize("Enter queue position: ") {
                                Some(pos) => {
                                    if let Err(err) = playback_queue.show_queue_song_details(pos) {
                                        println!("\n[!] {}", err);
                                    }
                                    println!("Press Enter to continue.");
                                    io::stdout().flush().unwrap();
                                    io::stdin().read_line(&mut String::new()).unwrap();
                                }
                                None => println!("\n[!] Please enter a valid position number."),
                            }
                        }
                    }
                    _ => println!("\n[!] Invalid source option."),
                }
            }
            Some(5) => {
                display_available_songs(&available_songs);
                println!("Press Enter to continue.");
                io::stdout().flush().unwrap();

                io::stdin().read_line(&mut String::new()).unwrap();
            }
            Some(6) => {
                println!("\nExiting Music Queue Manager. Goodbye!");
                break;
            }
            _ => println!("\n[!] Invalid menu option. Please select 1-6."),
        }
    }
}
