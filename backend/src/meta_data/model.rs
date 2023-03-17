use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct VideoUri {
    pub uri: String,
}

#[derive(Serialize,Debug,sqlx::FromRow)]
pub struct MetaData {
     uri: String,
     audio_codec: String,
     max_bitrate: i32,
     bitrate: i32,
     language_code: String,
     title: String,
     artist: String,
     album_artist: String,
     album: String,
     copyright: String,
     comment: String,
     description: String,
     encoder: String,
     genre: String,
     image: String,
     keywords: String,
     private_qt_tag: String,
     container_format: String,
     video_codec: String,
     duration: String,
}