use arboard::{Clipboard, LinuxClipboardKind, SetExtLinux};
use clap::Parser;
use feedme::find_feed_url;
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Copy to clipboard")]
    copy: bool,

    url: String,
}

fn main() {
    let args = Args::parse();

    let url = &args.url;

    let body = match ureq::get(url).call() {
        Ok(mut x) => match x.body_mut().read_to_string() {
            Ok(s) => s,
            Err(_) => process::exit(1),
        },
        Err(_) => process::exit(1),
    };
    if let Some(feed_url) = find_feed_url(url, &body) {
        println!("{}", feed_url);

        if args.copy {
            println!("Copied to clipboard");
            let mut clipboard = Clipboard::new().unwrap();
            clipboard
                .set()
                .clipboard(LinuxClipboardKind::Clipboard)
                .wait()
                .text(feed_url)
                .unwrap();
        }
    } else {
        process::exit(1);
    }
}
