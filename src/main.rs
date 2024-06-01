use feedme::find_feed_url;
use std::env;
use std::process;

fn main() -> Result<(), ureq::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: feedme [URL]");
        process::exit(1);
    }

    let url = &args[1];
    let body: String = ureq::get(url).call()?.into_string()?;
    if let Some(feed_url) = find_feed_url(url, &body) {
        println!("{}", feed_url);
    } else {
        process::exit(1);
    }
    Ok(())
}
