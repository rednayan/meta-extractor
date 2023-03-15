use gstreamer as gst;
use gstreamer_pbutils as gst_pbutils;
use gst::prelude::*;
use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;

use anyhow::Error;
use gst_pbutils::{prelude::*, DiscovererInfo, DiscovererStreamInfo};
use derive_more::{Display, Error};
pub struct MetaData {
    uri: String,
    audio_codec: String,
    max_bitrate: u32,
    bitrate: u32,
    language_code: String,
    title: String,
    artist: String,
    album_artist: String,
    album: String,
    copyright: String,
    comment: String,
    description: String,
    datetime: String,
    date: String,
    encoder: String,
    genre: String,
    image: String,
    keywords: String,
    private_qt_tag: String,
    container_format: String,
    video_codec: String 
}
#[derive(Debug, Display, Error)]
#[display(fmt = "Discoverer error {_0}")]
struct DiscovererError(#[error(not(source))] &'static str);

fn print_tags(info: &DiscovererInfo) {
    println!("Tags:");

    let tags = info.tags();
    match tags {
        Some(taglist) => {
            println!("Tags:");
            for tag in taglist.iter() {
                let tag_str = tag.1.transform::<String>();
                match tag_str {
                    Ok(value) => { 
                        let str_value = value.get::<&str>().unwrap();
                        let mut null_terminated_string = str_value.to_owned();
                        null_terminated_string.push('\0');
                        let c_str = std::ffi::CStr::from_bytes_with_nul(null_terminated_string.as_bytes())
                        .map_err(|e| format!("Error creating CStr from byte array: {}", e)).unwrap();
                        let rust_string = c_str.to_string_lossy().to_string(); 
                        println!("{}",rust_string);
                    },
                    Err(_) => println!("there has been an error")
                }
            }
        }
        None => {
            println!("no tags");
        }
    }
}

fn print_stream_info(stream: &DiscovererStreamInfo) {
    println!("Stream: ");
    println!("  Stream id: {}", stream.stream_id());
    let caps_str = match stream.caps() {
        Some(caps) => caps.to_string(),
        None => String::from("--"),
    };
    println!("  Format: {caps_str}");
}

fn print_discoverer_info(info: &DiscovererInfo) -> Result<(), Error> {
    println!("URI: {}", info.uri());
    println!("Duration: {}", info.duration().display());
    print_tags(info);
    print_stream_info(
        &info
            .stream_info()
            .ok_or(DiscovererError("Error while obtaining stream info"))?,
    );

    let children = info.stream_list();
    println!("Children streams:");
    for child in children {
        print_stream_info(&child);
    }

    Ok(())
}

fn run_discoverer() -> Result<(), Error> {
    gst::init()?;
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: discoverer uri");
        std::process::exit(-1)
    };
    let timeout: gst::ClockTime = gst::ClockTime::from_seconds(15);
    let discoverer = gst_pbutils::Discoverer::new(timeout)?;
    let info = discoverer.discover_uri(uri)?;
    print_discoverer_info(&info)?;
    Ok(())
}

pub fn example_main() {
    match run_discoverer() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {e}"),
    }
}

