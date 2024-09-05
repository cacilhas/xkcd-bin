mod comic;

use std::io::Cursor;

pub use comic::Comic;
use eyre::Result;
use image::ImageFormat;
use kitty_image::{Action, ActionPut, ActionTransmission, Command, Format, Medium, WrappedCommand};


pub async fn fetch_comic(num: u32) -> Result<()> {
    let comic = Comic::fetch(num).await?;
    show_comic(&comic).await
}

pub async fn fetch_latest() -> Result<()> {
    let comic = Comic::latest().await?;
    show_comic(&comic).await
}

pub async fn fetch_random() -> Result<()> {
    let comic = Comic::random().await?;
    show_comic(&comic).await
}


async fn show_comic(comic: &Comic) -> Result<()> {
    println!(
        "\x1b[33;1m{}\x1b[0m \x1b[31m{}\x1b[0m\n",
        &comic.title,
        &comic.date.format("%Y-%m-%d"),
    );
    println!("\x1b[37;40m{}\x1b[0m\n", &comic.transcript);
    let img = download_img(&comic.img).await?;

    let action = Action::TransmitAndDisplay(
        ActionTransmission {
            format: Format::Png,
            medium: Medium::Direct,
            ..Default::default()
        },
        ActionPut {
            move_cursor: true,
            ..Default::default()
        }
    );
    let mut command = Command::new(action);
    command.payload = img.into();
    let command = WrappedCommand::new(command);
    command.send_chunked(&mut std::io::stdout())?;
    println!("\n{}", &comic.link);

    Ok(())
}

async fn download_img(url: &str) -> Result<Vec<u8>> {
    let resp = reqwest::get(url).await?;
    let payload = resp.bytes().await?.iter().copied().collect();

    if url.ends_with(".png") {
        return Ok(payload);
    }

    let mut cursor = Cursor::new(payload);
    let image = image::load(&mut cursor, ImageFormat::Jpeg)?;
    let mut res: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut res), ImageFormat::Png)?;
    Ok(res)
}
