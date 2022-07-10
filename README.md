# m3u8-downloader

简单实现的 m3u8 下载器

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

- [x] 断点续传
