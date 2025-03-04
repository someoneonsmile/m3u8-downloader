# m3u8-downloader

cli m3u8-downloader

## Screenshot

![preview.png](https://github.com/someoneonsmile/m3u8-downloader/blob/main/img/preview.png?raw=true)

## Usage

```
Usage: m3u8-downloader [OPTIONS] --dest <DEST> <--source <SOURCE>|--url <URL>>

Options:
  -d, --dest <DEST>      dest path
  -w, --worker <WORKER>  parallel worker num [default: 20]
  -s, --source <SOURCE>  source used for from file
      --url <URL>        url to download
  -h, --help             Print help
  -V, --version          Print version
```

## Required

- ffmpeg

## Features

- [x] resume from a breakpoint

## TODO

- [ ] read from file
- [x] url parse
- [x] http/socks proxy
- [x] gzip / brotli / zstd support
- [ ] cookie
- [ ] url capture
- [x] m3u8 parse
  - [x] master playlist
  - [x] variants choice
    - [x] friendly tips
- [x] base64 decode
- [ ] aes decrypted
- [ ] hls - dash

- [x] github action
- [x] shell completions and man help
- [x] edition 2024 - async closure
- [x] clippy lint
