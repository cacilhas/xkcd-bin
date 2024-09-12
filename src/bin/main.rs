extern crate clap;
extern crate tokio;

use clap::{ArgAction, Parser};
use eyre::{eyre, Result};
use xkcd_bin::Comic;

const BEST_EVER: u32 = 162;


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let comic = match Params::from(&args)? {
        Params::Latest => Comic::latest().await?,
        Params::Random => Comic::random().await?,
        Params::Num(num) => Comic::fetch(num).await?,
    };
    if args.browse {
        comic.open()?;
    } else {
        comic.render().await?;
    }

    Ok(())
}


#[derive(Debug, Parser)]
#[command(name = "xkcd", author, about, version)]
struct Args {
    /// comic number | "latest" | "random"
    #[arg()]
    num: Option<String>,

    /// open comic in the default web browser
    #[arg(short, long, action = ArgAction::SetTrue)]
    browse: bool,
}

#[derive(Debug)]
enum Params {
    Latest,
    Random,
    Num(u32),
}

impl Params {

    fn from(args: &Args) -> Result<Self> {
        let num = args.num.to_owned().map(|num| num.to_lowercase());
        match num {
            Some(num) if num == "latest"    => Ok(Params::Latest),
            Some(num) if num == "random"    => Ok(Params::Random),
            Some(num) if num == "best"      => Ok(Params::Num(BEST_EVER)),
            Some(num) if num == "best-ever" => Ok(Params::Num(BEST_EVER)),
            Some(num) if num == "best_ever" => Ok(Params::Num(BEST_EVER)),
            Some(num) => {
                let num: u32 = num.parse().map_err(|_| eyre!("number expected, got {}", num))?;
                Ok(Params::Num(num))
            }
            None => Ok(Params::Latest),
        }
    }
}
