use actix_web::{post,web,Responder,HttpResponse};
use std::{collections::HashMap, hash::Hash, ptr::hash};

use super::model::{VideoUri};
use crate::AppState;



#[post("/meta_data")]
async fn post_meta_data(pool_data: web::Data<AppState>,param_obj: web::Json<VideoUri>) -> impl Responder {
    // let pool = &pool_data.pool;
    // let query: String = format!("INSERT INTO media_files (
    //     uri, audio_codec, max_bitrate, bitrate, language_code, title, artist,
    //     album_artist, album, copyright, comment, description, datetime,
    //     date, encoder, genre, image, keywords, private_qt_tag, container_format,
    //     video_codec
    // ) VALUES (
    //     '{}', 'AAC', 320, 192, 'en', 'My Awesome Video',
    //     'John Smith', 'The Band', 'The Best Album', '© 2023 My Company', 'This is a great video',
    //     'A longer description of the video and its contents', '2023-03-15 14:30:00',
    //     '2023-03-15', 'FFmpeg', 'Action', 'http://example.com/my_video_thumbnail.jpg',
    //     'video, awesome, cool', 'my-private-tag', 'MP4', 'H.264'
    // );",param_obj.uri);
    // sqlx::query(&query).execute(&*pool).await.expect("there is an error");

    let mut hash_tag: HashMap<&String,&String> = HashMap::new(); 
    let tags = extractor::extract(&param_obj.uri);
    for tag in tags.iter() {
        hash_tag.insert(&tag.meta_title, &tag.meta_value);
    }
    return HttpResponse::Ok().json(hash_tag);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_meta_data);
}