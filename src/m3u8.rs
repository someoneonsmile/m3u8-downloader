use std::future::IntoFuture;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use m3u8_rs::MediaPlaylist;
use m3u8_rs::Playlist;
use url::Url;

use crate::request;

pub(crate) async fn parse(base_uri: &Url, input: &[u8]) -> Result<MediaPlaylist> {
    let c = _parse(
        base_uri,
        input,
        |items| Ok(dialoguer::Select::new().items(items).interact()?),
        |uri| async move { request::get_bytes(uri).await },
    )
    .await?;
    Ok(c)
}

async fn _parse<S, D, Fut>(
    base_uri: &Url,
    input: &[u8],
    select_fn: S,
    download_fn: D,
) -> Result<MediaPlaylist>
where
    S: Fn(&[Url]) -> Result<usize> + Send + Sync + 'static,
    D: Fn(Url) -> Fut + Send + Sync + 'static,
    Fut: IntoFuture<Output = Result<Vec<u8>>> + 'static,
{
    let parsed = m3u8_rs::parse_playlist_res(input).map_err(|e| anyhow!("{:?}", e))?;
    match parsed {
        Playlist::MasterPlaylist(pl) => {
            let uris: Vec<Url> = pl
                .variants
                .iter()
                .filter(|v| !v.is_i_frame)
                .filter_map(|v| base_uri.join(&v.uri).ok())
                .collect();
            let uris = Arc::new(uris);
            let i = tokio::task::spawn_blocking({
                let uris = uris.clone();
                move || select_fn(&uris)
            })
            .await??;
            let uri = &uris[i];
            let content = download_fn(uri.clone()).await?;
            let pl: MediaPlaylist =
                m3u8_rs::parse_media_playlist_res(&content).map_err(|e| anyhow!("{:?}", e))?;
            Ok(pl)
        }
        Playlist::MediaPlaylist(pl) => Ok(pl),
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::fs;
    use url::Url;

    #[tokio::test]
    async fn parse() -> Result<()> {
        let d = fs::read_dir("data")?;
        let base_uri = Url::parse("http://some.com/")?;
        for f in d.into_iter() {
            let f = f?;
            let a: Vec<u8> = fs::read(dbg!(f.path()))?;
            super::_parse(
                &base_uri,
                &a,
                |_items| Ok(0),
                |_url| async {
                    let content = tokio::fs::read("data/mediaplaylist.m3u8").await?;
                    Ok(content)
                },
            )
            .await?;
        }
        Ok(())
    }
}
