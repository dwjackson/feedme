use scraper::{Html, Selector};
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
    }
    Ok(())
}

fn find_feed_url(base_url: &str, body: &str) -> Option<String> {
    let feed_path = find_feed_path(body);
    if feed_path.is_none() {
        return None;
    }
    let feed_path = feed_path.unwrap();
    let feed_url = if feed_path.starts_with("/") {
        let mut s = String::new();
        s.push_str(base_url);
        if !s.ends_with("/") {
            s.push('/');
        }
        s.push_str(&feed_path[1..]);
        s
    } else {
        feed_path
    };
    Some(feed_url.to_string())
}

fn find_feed_path(page_body: &str) -> Option<String> {
    let doc = Html::parse_document(page_body);
    let feed_mime_types: [&str; 2] = ["rss+xml", "atom+xml"];
    for mime_type in feed_mime_types.iter() {
        let selector_text = format!(
            "link[rel=\"alternate\"][type=\"application/{}\"]",
            mime_type
        );
        let selector = Selector::parse(&selector_text).unwrap();
        for elem in doc.select(&selector) {
            return Some(elem.attr("href").unwrap().to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_feed_path_for_atom_with_full_url_in_href() {
        let body = "<html><body><link rel=\"alternate\" type=\"application/atom+xml\" href=\"http://example.com/feed.xml\"></body></html>";
        let opt = find_feed_path(body);
        match opt {
            Some(u) => assert_eq!(u, "http://example.com/feed.xml"),
            None => panic!("Wrong feed URL"),
        }
    }

    #[test]
    fn test_no_feed_path() {
        let body = "<html><body></body></html>";
        let opt = find_feed_path(body);
        assert!(opt.is_none());
    }

    #[test]
    fn test_find_feed_url_with_relative_rss_url_with_trailing_base_url_slash() {
        let body = "<html><body><link rel=\"alternate\" type=\"application/rss+xml\" href=\"/feed.xml\"></body></html>";
        let opt = find_feed_url("http://example.com/", body);
        match opt {
            Some(url) => assert_eq!(url, "http://example.com/feed.xml"),
            None => panic!("No URL found"),
        }
    }

    #[test]
    fn test_find_feed_url_with_relative_rss_url_without_trailing_base_url_slash() {
        let body = "<html><body><link rel=\"alternate\" type=\"application/rss+xml\" href=\"/feed.xml\"></body></html>";
        let opt = find_feed_url("http://example.com", body);
        match opt {
            Some(url) => assert_eq!(url, "http://example.com/feed.xml"),
            None => panic!("No URL found"),
        }
    }
}
