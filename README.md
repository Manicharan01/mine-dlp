# Description

This rust package is a wrapper around [yt-dlp](https://github.com/yt-dlp/yt-dlp) to Download audio/video files to your desired folder with no hassle.

## Installation

You can install this package using cargo, like this
```bash
cargo install mine-dlp
```
you can install this package in any Operating System, with cargo installed. You can refer to [this](https://www.rust-lang.org/tools/install) website to install cargo. And you should install yt-dlp too, because this package is a wrapper around [yt-dlp](https://github.com/yt-dlp/yt-dlp).
You can refer to above referred link to install [yt-dlp](https://github.com/yt-dlp/yt-dlp).
You should also install `gcc` in linux and in windows you should in `Visual Studio 2019 or higher`, for Rust compiler to work.

## Execution

You can execute it by running `mine-dlp`, but you should have config file in your `$HOME` directory by the name of `.mine-dlp` and the file should contain
```bash
browser="name of the browser in which youtube is logged in, quotes are not needed"(yt-dlp cannot get audio/video without this option)
downloads=/path/to/save/the/audio or video/
```
If you do not want to go through hassle, you can just execute it and give details for the first when it asked.

## Issues

If there is a issue you can open a PR in GitHub, attaching a screenshot of the issue is appreciated