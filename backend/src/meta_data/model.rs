use serde::{Serialize,Deserialize};
use chrono::prelude::*;

#[derive(Deserialize)]
pub struct VideoUri {
    pub uri: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct MetaData {
//     uri: String,
//     audio_codec: String,
//     max_bitrate: u32,
//     bitrate: u32,
//     language_code: String,
//     title: String,
//     artist: String,
//     album_artist: String,
//     album: String,
//     copyright: String,
//     comment: String,
//     description: String,
//     datetime: DateTime<Local>,
//     date: DateTime<Local>,
//     encoder: String,
//     genre: String,
//     image: String,
//     keywords: String,
//     private_qt_tag: String,
//     container_format: String,
//     video_codec: String 
// }