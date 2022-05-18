test-run:
	rm -rf /tmp/m3u8-downloader*
	cargo run -- --url $(url) --dest $(dest)
