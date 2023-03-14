use gstreamer as gst;
use gst::prelude::*;

pub fn extract() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Build the pipeline
    let uri =
        "https://ia804505.us.archive.org/20/items/MIT6.006S20/MIT6_006S20_02_04_Lecture_1_300k.mp4";
    let pipeline = gst::parse_launch(&format!("playbin uri={uri}")).unwrap();
    
    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

       

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
