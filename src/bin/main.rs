extern crate clap;
extern crate tokio;

use clap::{ArgAction, Parser};
use eyre::Result;
use xkcd_bin::Comic;


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let params = Params::from(&args)?;
    if args.open {
        match params {
            Params::Latest => Comic::latest().await?.open()?,
            Params::Random => Comic::random().await?.open()?,
            Params::Num(num) => Comic::fetch(num).await?.open()?,
        }

    } else {
        match params {
            Params::Latest => xkcd_bin::fetch_latest().await?,
            Params::Random => xkcd_bin::fetch_random().await?,
            Params::Num(num) => xkcd_bin::fetch_comic(num).await?,
        }
    }

    Ok(())
}


#[derive(Debug, Parser)]
#[command(
    author = "Montegasppα Cacilhας <montegasppa@cacilhas.info>",
    about = "Fetches xkcd.com comics",
    name ="xkcd",
)]
struct Args {
    #[arg(help = "comic number | latest | random")]
    num: Option<String>,

    #[arg(short, long, name = "OPEN ON BROWSER", action = ArgAction::SetTrue)]
    open: bool,
}

#[derive(Debug)]
enum Params {
    Latest,
    Random,
    Num(u32),
}

impl Params {

    fn from(args: &Args) -> Result<Self> {

        match &args.num {
            Some(num) if num == "latest" => Ok(Params::Latest),
            Some(num) if num == "random" => Ok(Params::Random),
            Some(num) => {
                let num: u32 = num.parse()?;
                Ok(Params::Num(num))
            }
            None => Ok(Params::Latest),
        }
    }
}
