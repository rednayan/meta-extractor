# meta-extractor
This is a video metadata extractor tool written in Rust that uses GStreamer to encode videos and Actix web to create a server and endpoints to access the extracted metadata.

FeaturesExtracts video metadata (duration, format, resolution, etc.) from uploaded videosSupports various video formats including MP4, AVI, and MOVUses GStreamer to encode videos for faster processingBuilt with Actix web framework for easy setup and customizationInstallation

To use this tool, you will need to have Rust installed on your system. You can download Rust from the official website: https://www.rust-lang.org/tools/install

Once Rust is installed, you can clone this repository and run the following command to build and run the server:

`cargo run`

This will start the server at http://localhost:6942

>Usage

To extract metadata from a video, you can use the following endpoint:

`POST /metadata`

You can upload a video file using the file field in a multipart form data request.

The server will respond with a JSON object containing the extracted metadata:

>json

`{ "duration": 3600, 
"format": "mp4", 
"resolution": "1920x1080", ... }`

>Contributing

If you would like to contribute to this project, feel free to submit a pull request. Before submitting a pull request, please make sure that your changes are consistent with the coding style of the project and that all tests pass.

