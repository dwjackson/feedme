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
        Ok(mut x) => match x.body_mut().read_to_string() {
            Ok(s) => s,
            Err(_) => process::exit(1),
        },
        Err(_) => process::exit(1),
    };
    if let Some(feed_url) = find_feed_url(url, &body) {
        println!("{}", feed_url);
    } else {
        process::exit(1);
    }
}
