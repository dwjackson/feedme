use feedme::find_feed_url;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: feedme [URL]");
        process::exit(1);
    }

    let url = &args[1];
    let body = match ureq::get(url).call() {
        Ok(x) => x.into_string().unwrap(),
        Err(_) => process::exit(1),
    };
    if let Some(feed_url) = find_feed_url(url, &body) {
        println!("{}", feed_url);
    } else {
        process::exit(1);
    }
}
