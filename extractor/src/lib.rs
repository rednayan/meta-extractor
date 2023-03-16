use gstreamer as gst;
use gstreamer_pbutils as gst_pbutils;
use gst::prelude::*;

use anyhow::Error;
use gst_pbutils::{DiscovererInfo};
use derive_more::{Display, Error};
#[derive(Debug, Display, Error)]
#[display(fmt = "Discoverer error {_0}")]
struct DiscovererError(#[error(not(source))] &'static str);


fn get_tags(info: &DiscovererInfo) -> Vec<String> {
    let tags = info.tags();
    let taglist = match tags {
        Some(taglist) => taglist,
        None => std::process::exit(1)
    };
    let mut tags_vec :Vec<String> = Vec::new();
    for tag in taglist.iter() {
        let  tag_str = tag.1.transform::<String>();
        match tag_str {
            Ok(value) => { 
                let str_value = value.get::<&str>().unwrap();
                let mut null_terminated_string = str_value.to_owned();
                null_terminated_string.push('\0');
                let c_str = std::ffi::CStr::from_bytes_with_nul(null_terminated_string.as_bytes())
                .map_err(|e| format!("Error creating CStr from byte array: {}", e)).unwrap();
                let r_str = c_str.to_string_lossy().to_string(); 
                tags_vec.push(r_str);
                format!("Success extracting")
            },
            Err(_) => format!("ERROR: Invalid extraction")
        };
    }
    return tags_vec;
}

fn run_discoverer(uri:&String) -> Result<Vec<String>, Error> {
    gst::init()?;
    let timeout: gst::ClockTime = gst::ClockTime::from_seconds(15);
    let discoverer = gst_pbutils::Discoverer::new(timeout)?;
    let info = discoverer.discover_uri(uri)?;
    let mut tag_vec = get_tags(&info);
    tag_vec.push(info.uri().to_string());
    tag_vec.push(info.duration().display().to_string());

    Ok(tag_vec)
}

pub fn extract(uri:&String) -> Vec<String> {
    match run_discoverer(uri) {
        Ok(vec_tag) => vec_tag,
        Err(_) => vec![String::from("ERROR 101:")]
    }
}

