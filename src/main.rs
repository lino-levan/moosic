use freya::prelude::*;
use rand::prelude::*;
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{fs::File, sync::LazyLock};
use types::Song;

mod types;

static SONGS: LazyLock<Mutex<Vec<Song>>> = LazyLock::new(|| Mutex::new(vec![]));
static SINK: LazyLock<Mutex<Option<Sink>>> = LazyLock::new(|| Mutex::new(None));

fn home_dir() -> PathBuf {
    std::env::home_dir().unwrap()
}

fn get_random_song_file() -> File {
    let songs = SONGS.lock().unwrap();
    let song = songs.choose(&mut rand::thread_rng()).unwrap();
    let path = home_dir().join(".moosic").join(&song.file);
    File::open(path).unwrap()
}

fn app() -> Element {
    let mut playing = use_signal(|| false);

    let onclickplay = move |_| {
        let sink = SINK.lock().unwrap();
        let sink = sink.as_ref().unwrap();
        let file = get_random_song_file();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
        sink.play();
        *playing.write() = true;
    };

    let onclickpause = move |_| {
        let sink = SINK.lock().unwrap();
        let sink = sink.as_ref().unwrap();
        sink.pause();
        *playing.write() = false;
    };

    // stop the current song and play a new one
    let onclickskip = move |_| {
        let sink = SINK.lock().unwrap();
        let sink = sink.as_ref().unwrap();
        sink.stop();
        let file = get_random_song_file();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
        sink.play();
        *playing.write() = true;
    };

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            if *playing.read() {
                label {
                    onclick: onclickpause,
                    "Pause"
                }
            } else {
                label {
                    onclick: onclickplay,
                    "Play"
                }
            },
            label {
                onclick: onclickskip,
                "Skip"
            }
        }
    )
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    SINK.lock()
        .unwrap()
        .replace(Sink::try_new(&stream_handle).unwrap());
    let song_json = std::fs::read_to_string(home_dir().join(".moosic/songs.json")).unwrap();
    let songs: Vec<Song> = serde_json::from_str(&song_json).unwrap();
    SONGS.lock().unwrap().extend(songs);
    launch(app);
}
