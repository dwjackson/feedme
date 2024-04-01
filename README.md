# FeedMe

Find RSS (and Atom) feed URLs when they're not obviously advertised.

## Usage

Given the URL of a page that you think might have an RSS feed:

```sh
$ feedme 'http://example.com/blog'
http::/example.com/feed.xml
```

## Copying the Link to the Clipboard

Since `feedme` outputs to stdout, you can pipe this output to a utility that
copies a string to the clipboard. 

### Wayland

```sh
$ feedme 'http://example.com/blog' | wl-copy
```

### X11

```sh
$ feedme 'http://example.com/blog' | xclip
``` 

### macOS

```sh
$ feedme 'http://example.com/blog' | pbcopy
``` 
