# m3u8-downloader

简单实现的 m3u8 下载器

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

- [x] 断点续传
