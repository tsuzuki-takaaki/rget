use std::{io::{BufWriter, Write}, fs::File};

pub async fn download(url: &str) -> Result<(), Box<dyn std::error::Error>> {
  let resp = reqwest::get(url).await?;
  println!("HTTP request sent... {}", resp.status());

  if resp.status().is_success() {
    let headers = resp.headers();
    let content_type = headers.get("content-type").unwrap();
    println!("Type: {}", content_type.to_str().unwrap());

    let file_name = url.split("/").last().unwrap();
    println!("Save to: {}", file_name);

    let mut file = BufWriter::new(File::create(file_name)?);
    file.write(&resp.bytes().await.unwrap())?;
  }
  Ok(())
}
