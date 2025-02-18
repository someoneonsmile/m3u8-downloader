# m3u8-downloader

cli m3u8-downloader

## Screenshot

![preview.png](https://github.com/someoneonsmile/m3u8-downloader/blob/main/img/preview.png?raw=true)

## Usage

```
USAGE:
    m3u8-downloader [OPTIONS] --url <URL> --dest <DEST>

OPTIONS:
    -d, --dest <DEST>        dest path
    -h, --help               Print help information
        --url <URL>          url to download
    -V, --version            Print version information
    -w, --worker <WORKER>    parallel worker num [default: 20]
```

## Features

- [x] resume from a breakpoint

## TODO

- [ ] url parse
- [ ] http/socks proxy
- [ ] cookie
- [ ] url capture
- [ ] m3u8 parse
  - [ ] master playlist
  - [ ] variants choice
- [ ] base64 decode
- [ ] aes decrypted
- [ ] hls - dash
