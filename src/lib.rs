use std::io::Cursor;

use chrono::{NaiveDate, Utc};
use eyre::Result;
use image::ImageFormat;
use kitty_image::{
    Action,
    ActionPut,
    ActionTransmission,
    Command,
    Format,
    WrappedCommand,
};
use random::Source;
use serde::Deserialize;


#[derive(Debug)]
pub struct Comic {
    pub date: NaiveDate,
    #[allow(dead_code)]
    pub num: u32,
    pub img: String,
    pub link: String,
    pub title: String,
    pub transcript: String,
}


impl Comic {
    pub async fn fetch(num: u32) -> Result<Self> {
        let comic = fetch_comic(Some(num)).await?;
        Ok(comic.into())
    }

    pub async fn latest() -> Result<Self> {
        let comic = fetch_comic(None).await?;
        Ok(comic.into())
    }

    pub async fn random() -> Result<Self> {
        let mut rnd = random::default(Utc::now().timestamp_millis() as u64);
        let latest = fetch_comic(None).await?;
        let num = (rnd.read::<u32>() % latest.num) + 1;
        let comic = fetch_comic(Some(num)).await?;
        Ok(comic.into())
    }

    pub fn open(&self) -> Result<()> {
        webbrowser::open(&self.link)?;
        Ok(())
    }

    pub async fn render(&self) -> Result<()> {
        println!(
            "\x1b[33;1m{}\x1b[0m \x1b[31m{}\x1b[0m\n",
            &self.title,
            &self.date.format("%Y-%m-%d"),
        );
        println!("\x1b[37;40m{}\x1b[0m\n", &self.transcript);
        let img = download_img(&self.img).await?;

        let action = Action::TransmitAndDisplay(
            ActionTransmission {
                format: Format::Png,
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
        println!("\n{}", &self.link);

        Ok(())
    }
}


impl From<RawComic> for Comic {
    fn from(value: RawComic) -> Self {

        let title = match &value.title {
            title if title.is_empty() => value.safe_title.to_owned(),
            title => title.to_owned(),
        };

        let transcript = match &value.transcript {
            transcript if transcript.is_empty() => value.alt.to_owned(),
            transcript => transcript.to_owned(),
        };

        let link = match &value.link {
            link if link.is_empty() => format!("https://xkcd.com/{}", value.num),
            link => link.to_owned(),
        };

        Self {
            title, transcript, link,
            date: value.date().unwrap_or(NaiveDate::MIN),
            num: value.num,
            img: value.img,
        }
    }
}


#[derive(Debug, Deserialize)]
struct RawComic {
    month: String,
    num: u32,
    link: String,
    year: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

impl RawComic {
    fn date(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(
            self.year.parse().ok()?,
            self.month.parse().ok()?,
            self.day.parse().ok()?,
        )
    }
}


async fn fetch_comic(num: Option<u32>) -> Result<RawComic> {
    let url = match num {
        Some(num) => format!("https://xkcd.com/{}/info.0.json", num),
        None => "https://xkcd.com/info.0.json".to_string(),
    };
    let resp = reqwest::get(url).await?;
    let comic: RawComic = resp.json().await?;
    Ok(comic)
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
