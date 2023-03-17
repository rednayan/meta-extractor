use actix_web::{post,web,Responder,HttpResponse};
use extractor::Meta;
use std::{collections::HashMap};

use super::model::{VideoUri};
use crate::AppState;


#[post("/meta_data")]
async fn post_meta_data(pool_data: web::Data<AppState>,param_obj: web::Json<VideoUri>) -> impl Responder {
    let pool = &pool_data.pool;

    let tags = extract_data(&param_obj.uri).await;

    let hash_tag: HashMap<String,String> = tags.iter().map(|tag| (tag.meta_title.clone(),tag.meta_value.clone())).collect();
    let null_default: String = "NULL".to_string();
    let date_default: String = "1979-01-01".to_string();

    let uri = hash_tag.get("uri").unwrap_or(&null_default);
    let audio_codec = hash_tag.get("audio-codec").unwrap_or(&null_default);
    let max_bitrate = hash_tag.get("maximum-bitrate").and_then(|v| v.parse().ok()).unwrap_or(0);
    let bitrate = hash_tag.get("bitrate").and_then(|v| v.parse().ok()).unwrap_or(0);
    let language_code = hash_tag.get("language-code").unwrap_or(&null_default);
    let title = hash_tag.get("title").unwrap_or(&null_default);
    let artist = hash_tag.get("artist").unwrap_or(&null_default);
    let album_artist = hash_tag.get("album-artist").unwrap_or(&null_default);
    let album = hash_tag.get("album").unwrap_or(&null_default);
    let copyright = hash_tag.get("copyright").unwrap_or(&null_default);
    let comment = hash_tag.get("comment").unwrap_or(&null_default);
    let description = hash_tag.get("description").unwrap_or(&null_default);
    let date = hash_tag.get("date").unwrap_or(&date_default);
    let encoder = hash_tag.get("encoder").unwrap_or(&null_default);
    let genre = hash_tag.get("genre").unwrap_or(&null_default);
    let image = hash_tag.get("image").unwrap_or(&null_default);
    let keywords = hash_tag.get("keywords").unwrap_or(&null_default);
    let private_qt_tag = hash_tag.get("private-qt-tag").unwrap_or(&null_default);
    let container_format = hash_tag.get("container-format").unwrap_or(&null_default);
    let video_codec = hash_tag.get("video-codec").unwrap_or(&null_default);
    let duration = hash_tag.get("duration").unwrap_or(&null_default);

    sqlx::query("INSERT INTO media_files (
        uri, audio_codec, max_bitrate, bitrate, language_code, title, artist,
        album_artist, album, copyright, comment, description,
        date, encoder, genre, image, keywords, private_qt_tag, container_format,
        video_codec,duration
    ) VALUES (
            ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?
        )")
        .bind(uri)
        .bind(audio_codec)
        .bind(max_bitrate)
        .bind(bitrate)
        .bind(language_code)
        .bind(title)
        .bind(artist)
        .bind(album_artist)
        .bind(album)
        .bind(copyright)
        .bind(comment)
        .bind(description)
        .bind(date)
        .bind(encoder)
        .bind(genre)
        .bind(image)
        .bind(keywords)
        .bind(private_qt_tag)
        .bind(container_format)
        .bind(video_codec)
        .bind(duration)
        .execute(&*pool).await.expect("there is an error");
            
    return HttpResponse::Ok().json(hash_tag);
   
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(post_meta_data);
}

pub async fn extract_data(param_obj: &String) -> Vec<Meta>{
    let tags = extractor::extract(param_obj);
    return tags;
}