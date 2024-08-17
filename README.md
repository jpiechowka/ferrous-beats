# Ferrous Beats

Ferrous Beats is a Rust music player (and many more).

Heavily inspired by this talk by Andrew Kelley (creator of Zig programming language): https://youtu.be/SCLrNqc9jdE

A few main features:

* Multiplatform (linux, windows and macos supported), with embedded UI for an easy setup.
* No system dependencies, download and run on any supported os (TODO)
* Allows easy download of tools like: `yt-dlp`, `ffmpeg` and `chromaprint`. No need to install anything.
* Audio / video downloads from multiple services using `yt-dlp`. You can download your favourite songs from YouTube (and
  many more).
* Music identification using `AcoustID` and `MusicBrainz`. No need to rename or edit tags of your files manually.
* Extensive labeling support for your files.
* Conversion of files using `ffmpeg`.
* DNS over HTTPS using Cloudflare for some added privacy
* TODO

## Usage (TODO)

```
Backend for Ferrous Beats music player

Usage: ferrous-beats-backend.exe [OPTIONS] <COMMAND>

Commands:
  run   Main application command used to run the server and serve the frontend
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Enable verbose logging
  -h, --help     Print help
  -V, --version  Print version
```

Run command:

```
Main application command used to run the server and serve the frontend

Usage: ferrous-beats-backend.exe run [OPTIONS]

Options:
  -p, --port <PORT>
          Server port [default: 13337]
      --host <HOST>
          Server host [default: 127.0.0.1]
      --disable-doh
          Disable DNS over HTTPS (DoH) for HTTP client. DoH provides some additional privacy compared to plain DNS
  -l, --library-dir <LIBRARY_DIR>
          Your file library directory [default: library]
  -t, --tools-download-dir <TOOLS_DOWNLOAD_DIR>
          Download directory for all the used tools (yt-dlp, ffmpeg, chromparint) [default: tools]
  -a, --audio-download-dir <AUDIO_DOWNLOAD_DIR>
          Download directory for audio files. They will be moved to library directory after successful download [default: downloads/music]
  -v, --video-download-dir <VIDEO_DOWNLOAD_DIR>
          Download directory for video files. They will be moved to library directory after successful download [default: downloads/videos]
  -h, --help
          Print help
```

## License

`ferrous-beats` is free, open source and permissively licensed! Except where noted (below and/or in individual files),
all code in this repository is dual-licensed under either:

* MIT License (`LICENSE-MIT` file or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (`LICENSE-APACHE` file or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard
in the Rust ecosystem.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
