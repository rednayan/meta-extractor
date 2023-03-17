use gstreamer as gst;
use gstreamer_pbutils as gst_pbutils;
use gst::prelude::*;

use anyhow::Error;
use gst_pbutils::{DiscovererInfo};
use derive_more::{Display, Error};
#[derive(Debug, Display, Error)]
#[display(fmt = "Discoverer error {_0}")]
struct DiscovererError(#[error(not(source))] &'static str);

#[derive(Debug)]
pub struct Meta {
    pub meta_title: String,
    pub meta_value: String
}

fn get_tags(info: &DiscovererInfo) -> Vec<Meta> {
    let tags = match info.tags() {
        Some(tag) => tag,
        None => {
            eprint!("ERROR: No tags found , try again");
            info.tags().unwrap()
        }
    };
    let mut tags_vec :Vec<Meta> = Vec::new();
    for tag in tags.iter() {
        let  tag_str = tag.1.transform::<String>();
        let meta_str = match tag_str {
            Ok(value) => { 
                let str_value = value.get::<&str>().unwrap();
                let mut null_terminated_string = str_value.to_owned();
                null_terminated_string.push('\0');
                let c_str = std::ffi::CStr::from_bytes_with_nul(null_terminated_string.as_bytes())
                .map_err(|e| format!("Error creating CStr from byte array: {}", e)).unwrap();
                let r_str = c_str.to_string_lossy().to_string(); 
                r_str
            },
            Err(_) => format!("ERROR: Invalid extraction")
        };
        tags_vec.push(Meta {
            meta_title: tag.0.to_string(),
            meta_value:  meta_str
        })
    }
    return tags_vec;
}

fn run_discoverer(uri:&String) -> Result<Vec<Meta>, Error> {
    gst::init()?;
    let timeout: gst::ClockTime = gst::ClockTime::from_seconds(60);
    let discoverer = gst_pbutils::Discoverer::new(timeout)?;
    let info = discoverer.discover_uri(uri)?;
    let mut tag_vec: Vec<Meta> = get_tags(&info);
    tag_vec.push(Meta {
        meta_title:"uri".to_string(),
        meta_value:info.uri().to_string()
    });
    tag_vec.push(Meta { meta_title: "duration".to_string(), meta_value: info.duration().display().to_string() });
    Ok(tag_vec)
}

pub fn extract(uri:&String) -> Vec<Meta> {
    match run_discoverer(uri) {
        Ok(vec_tag) => vec_tag,
        Err(_) => vec![Meta{
            meta_title:"error value".to_string(),
            meta_value:"error value".to_string()
        }]
    }
}

