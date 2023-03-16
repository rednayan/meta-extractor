use actix_web::{post,web,Responder,HttpResponse};
use extractor::Meta;
use std::{collections::HashMap};

use super::model::{VideoUri};
use crate::AppState;


#[post("/meta_data")]
async fn post_meta_data(pool_data: web::Data<AppState>,param_obj: web::Json<VideoUri>) -> impl Responder {
    let pool = &pool_data.pool;

    let mut hash_tag: HashMap<String,String> = HashMap::new(); 
    let tags = extract_data(&param_obj.uri).await;
    for tag in tags.iter() {
        hash_tag.insert(tag.meta_title.clone(), tag.meta_value.clone());  
    }
    let mut max_bitrate: u32 = 0;
    let mut bitrate: u32 = 0;
    let mut uri: String = String::new();
    let mut audio_codec: String = String::new();
    let mut language_code: String = String::new();
    let mut title: String = String::new();
    let mut artist: String = String::new();
    let mut album_artist: String = String::new();
    let mut album: String = String::new();
    let mut copyright: String = String::new();
    let mut comment: String = String::new();
    let mut description: String = String::new();
    let mut date: String = String::new();
    let mut encoder: String = String::new();
    let mut genre: String = String::new();
    let mut image: String = String::new();
    let mut keywords: String = String::new();
    let mut private_qt_tag: String = String::new();
    let mut container_format: String = String::new();
    let mut video_codec: String = String::new();
    let mut duration: String = String::new();
   

    if let Some(u) = hash_tag.get("uri") {
        uri.push_str(u);
    }else {
        uri.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("audio-codec") {
        audio_codec.push_str(u);
    }else {
        audio_codec.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("maximum-bitrate") {
        max_bitrate = u.parse::<u32>().unwrap();
    }
    if let Some(u) = hash_tag.get("bitrate") {
        bitrate = u.parse::<u32>().unwrap();
    }
    if let Some(u) = hash_tag.get("language-code") {
        language_code.push_str(u);
    }else {
        language_code.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("title") {
        title.push_str(u);
    }else {
        title.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("artist") {
        artist.push_str(u);
    }else {
        artist.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("album-artist") {
        album_artist.push_str(u);
    }else {
        album_artist.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("album") {
        album.push_str(u);
    }else {
        album.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("copyright") {
        copyright.push_str(u);
    }else {
        copyright.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("comment") {
        comment.push_str(u);
    }else {
        comment.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("description") {
        description.push_str(u);
    }else {
        description.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("date") {
        date.push_str(u);
    }else {
        date.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("encoder") {
        encoder.push_str(u);
    }else {
        encoder.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("genre") {
        genre.push_str(u);
    }else {
        genre.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("image") {
        image.push_str(u);
    }else {
        image.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("keywords") {
        keywords.push_str(u);
    }else {
        keywords.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("private-qt-tag") {
        private_qt_tag.push_str(u);
    }else {
        private_qt_tag.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("container-format") {
        container_format.push_str(u);
    }else {
        container_format.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("video-codec") {
        video_codec.push_str(u);
    }else {
        video_codec.push_str("NULL");
    }
    if let Some(u) = hash_tag.get("duration") {
        duration.push_str(u);
    }else {
        duration.push_str("NULL");
    }

    sqlx::query("INSERT INTO media_files (
        uri, audio_codec, max_bitrate, bitrate, language_code, title, artist,
        album_artist, album, copyright, comment, description,
        date, encoder, genre, image, keywords, private_qt_tag, container_format,
        video_codec,duration
    ) VALUES (
            ?, ?, ?, ?, ?, ?,
            ?, ?, ?, ?, ?,
            ?,
            ?, ?,?,?,
            ?, ?, ?, ?,?
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