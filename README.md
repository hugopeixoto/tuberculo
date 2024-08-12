# Tuberculo

[![REUSE status](https://api.reuse.software/badge/github.com/hugopeixoto/tuberculo)](https://api.reuse.software/info/github.com/hugopeixoto/tuberculo)

![Tuberculo](./logo.svg)

 Minimal Youtube archival tool. Inspired by tubearchivist.


## Background

I wanted a tool to archive the videos in my youtube history, because some of
them tend to be taken down. I found [tuberarchivist](https://www.tubearchivist.com/),
but when I noticed it required both ElasticSearch and Redis, I decided to make a
lightweight version using SQLite3.


## Install

```sh
cargo build --release
```

`tuberculo` requires `yt-dlp` to be installed on the system this is going to run. The binary
will write to `db/` and `videos/` (relative to the CWD). It listens on `0.0.0.0:3000`.


## Contributing

Feel free to open any issues with questions or bugs you find on the forge this is hosted on.


## License

[AGPL-3.0-only](./LICENSES/AGPL-3.0-only) © Hugo Peixoto
