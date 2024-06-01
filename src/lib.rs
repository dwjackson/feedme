use scraper::{Html, Selector};
use url::Url;

pub fn find_feed_url(base_url: &str, body: &str) -> Option<String> {
    let url = match Url::parse(base_url) {
        Ok(u) => u,
        Err(_) => {
            return None;
        }
    };
    let feed_path = find_feed_path(body);
    if feed_path.is_none() {
        return None;
    }
    let feed_path = feed_path.unwrap();
    let feed_url = if feed_path.starts_with("/") {
        format!(
            "{}://{}/{}",
            url.scheme(),
            url.host_str().unwrap(),
            &feed_path[1..]
        )
    } else {
        feed_path
    };
    Some(feed_url)
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

    #[test]
    fn test_find_feed_when_feed_is_not_subpath_of_url() {
        let url = "http://example.com/?page=blog.php";
        let body = "<html><body><link rel=\"alternate\" type=\"application/atom+xml\" href=\"/feed.xml\"></body></html>";
        let opt = find_feed_url(url, body);
        match opt {
            Some(u) => assert_eq!(u, "http://example.com/feed.xml"),
            None => panic!("Wrong feed URL"),
        }
    }
}
