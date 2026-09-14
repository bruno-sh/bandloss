use std::env;

mod filesystem;
mod net;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        eprintln!("error: missing arguments, must be 2 or more.");
        println!("usage: bandloss <bandcamp-url>");
        std::process::exit(1);
    }

    let client = net::build_client()?;

    for url in arguments[1..arguments.len()].into_iter() {
        let html = net::fetch_website(&client, &url).await?;
        let html = html.replace("&quot;", "\"").replace("&amp;", "&");

        let track_url = search_url(&html)?;
        let track_name = url.split('/').last().unwrap_or("Unknown Track");

        save_track(&client, track_url, track_name).await?;
    }

    Ok(())
}

fn search_url(text: &str) -> Result<&str, &'static str> {
    let start_pos = text
        .find("https://t4.bcbits.com/stream/")
        .ok_or("error: the cdn url wasn't found.")?;
    let rest = &text[start_pos..];
    let end_pos = rest
        .find('"')
        .ok_or("error: the cdn delimiter wasn't found.")?;

    let url = &rest[..end_pos];
    Ok(url)
}

async fn save_track(client: &reqwest::Client, url: &str, name: &str) -> Result<(), reqwest::Error> {
    let bytes = net::fetch_track(client, url).await?;
    filesystem::create_file(bytes, name);

    Ok(())
}
