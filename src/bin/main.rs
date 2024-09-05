extern crate clap;
extern crate tokio;

use clap::{ArgAction, Parser};
use eyre::Result;
use xkcd_bin::Comic;

const BEST_EVER: u32 = 162;


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let params = Params::from(&args)?;
    if args.open {
        eprintln!("\x1b[31m--open\x1b[0m is deprecated, use \x1b[32;1m--browse\x1b[0m instead")
    }
    let comic = match params {
        Params::Latest => Comic::latest().await?,
        Params::Random => Comic::random().await?,
        Params::Num(num) => Comic::fetch(num).await?,
    };
    if args.browse || args.open {
        comic.open()?;
    } else {
        comic.render().await?;
    }

    Ok(())
}


#[derive(Debug, Parser)]
#[command(
    author = "Montegasppα Cacilhας <montegasppa@cacilhas.info>",
    about = "Display Xkcd.com comics in Kitty Terminal or in the default web browser.",
    name = "xkcd",
)]
struct Args {
    #[arg(help = "comic number | \"latest\" | \"random\"")]
    num: Option<String>,

    #[arg(short, long, action = ArgAction::SetTrue, help = "deprecated, use --browse instead")]
    open: bool,
    #[arg(short, long, action = ArgAction::SetTrue, help = "open comic in the default web browser")]
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
                let num: u32 = num.parse()?;
                Ok(Params::Num(num))
            }
            None => Ok(Params::Latest),
        }
    }
}
