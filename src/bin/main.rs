extern crate clap;
extern crate tokio;

use clap::Parser;
use eyre::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let args = Params::from(&Args::parse())?;
    match args {
        Params::Latest => xkcd_bin::fetch_latest().await?,
        Params::Random => xkcd_bin::fetch_random().await?,
        Params::Num(num) => xkcd_bin::fetch_comic(num).await?,
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

    // TODO: open on browser
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
