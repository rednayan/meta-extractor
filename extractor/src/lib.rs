#[tokio::main]
pub async fn extract() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://ia804505.us.archive.org/20/items/MIT6.006S20/MIT6_006S20_02_04_Lecture_1_300k.mp4";
    let mut response = reqwest::get(url).await?;

    // let mut file = BufWriter::new(File::create("video.mp4")?);

    let mut total_size = 0;
    while let Some(chunk) = response.chunk().await? {
        let chunk_size = chunk.len();
        total_size += chunk_size;
    }
    println!("Total size: {}", total_size);

    Ok(())
}