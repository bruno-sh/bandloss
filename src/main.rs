use std::env;

mod filesystem;
mod net;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!(
            "usage: bandloss.x86_64 https://macdemarco.bandcamp.com/track/moonlight-on-the-river"
        );
        panic!("error: missing arguments!")
    }

    let url = &arguments[1];

    let client = net::build_client()?;
    let html = net::fetch_website(&client, &url).await?;

    let html = html.replace("&quot;", "\"").replace("&amp;", "&");

    let track_url = search_url(&html)?;
    save_track(&client, track_url).await?;

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

async fn save_track(client: &reqwest::Client, url: &str) -> Result<(), reqwest::Error> {
    let bytes = net::fetch_track(client, url).await?;
    let track_name = url.split('/').last().unwrap_or("unknown track");

    filesystem::create_file(bytes, track_name);
    Ok(())
}
