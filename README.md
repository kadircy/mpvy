# mpvy

[![asciicast](https://asciinema.org/a/uCgJdnVdi56KY0CLUnd1vYY6W.svg)](https://asciinema.org/a/uCgJdnVdi56KY0CLUnd1vYY6W)

MPVY is a free and open-source terminal-based YouTube audio player written in Rust. It leverages the MPV media player to stream YouTube content as audio, supporting a wide range of media formats and providing a fast, lightweight, and customizable experience for users.

## Installation

You can run the binary with `mpvy`.
Also you need to download `mpv` and `yt-dlp` for running **mpvy** correctly.

### With package manager

Sorry, there is currently no option to install using a package manager. For now, please install MPVY from source or download the latest release from GitHub.

### Manually

1. Ensure that Rust and Cargo are correctly installed on your system.
2. Clone the Github repository.
```bash
git clone https://github.com/kadircy/mpvy
```
3. Navigate to the project directory:
```bash
cd mpvy
```
4. Compile the release binary:
```bash
cargo build --release
```

You can now run the binary with: `./target/release/mpvy`

## Usage
The binary is named `mpvy`

Running `mpvy` without arguments will prompt you for an input, allowing you to enter search queries for YouTube. You can search for multiple queries at once by separating them with commas. After that, **mpvy** will search for the videos and download them to your local machine (if they are not already installed). Next, **mpvy** will use `yt-dlp` to fetch video information and download the audio. It will then automatically launch **mpv** with the necessary arguments to play the audio correctly. Finally, you will hear the audio system-wide.

**mpvy** currently doesn't support built-in controls like play, pause, and others. However, you can manage audio playback **customly** using IPC. **mpvy** automatically sets the IPC socket path for the played audio to `$XDG_CONFIG_HOME/mpv/socket`. To control the audio in **mpvy** (mpv instances that running audios), simply interact with this IPC socket path.

## Playlist
**mpvy** now supports local playlists, allowing you to play multiple audio tracks repeatedly without having to enter the query each time. To save a playlist, use the `--save-playlist <name>` argument and enter your video queries as usual. **mpvy** will store these queries in a file located at `$XDG_CONFIG_HOME/mpvy/playlist/<name>`. To play a saved playlist, simply use the `!playlist` prefix and type your playlist name in query. Like this `!playlist example`, and **mpvy** will handle playback seamlessly.

## Cava
**mpvy** has built-in support for displaying **cava** (a console-based audio visualizer) while playing audio. If you have **cava** installed on your system, you can use the `--cava` argument to automatically launch cava when the audio starts. Once all audio has finished playing, **cava** will be closed automatically.

## Configuration
This document provides an overview of the configuration options for the `mpvy` project.
The configuration file located at `$XDG_CONFIG_DIR/mpvy/config.toml` (`~/.config/mpvy/config.toml`)

### `max_file_count`
Sets the maximum number of audio files to be saved. Default: `15`

### `audio_quality`
Defines the audio quality for downloads using `yt-dlp`. `0` is the best and `10` is the worst. Default: `0`

### `concurrent_fragments`
Specifies the number of concurrent fragments for downloading audio using `yt-dlp`. Default: `4`

## Logs
You can access both **mpvy** and **mpv** logs in the `$XDG_CONFIG_HOME/mpvy/log` directory. Please note that these logs are overwritten from scratch with every **mpvy** command, so previous logs are deleted each time.

## Contributing

Contributions to **mpvy** are welcome and appreciated! If you'd like to contribute, please fork the repository, make your changes, and submit a pull request. Before submitting, ensure your code adheres to the existing style (run `cargo fmt` and resolve any warnings) and includes any necessary tests. If you encounter bugs or have feature suggestions, feel free to open an issue. Thank you for your support!

## Changelog
For more details on updates and changes, please refer to the [CHANGELOG](./CHANGELOG.md) page.

## License
This project is licensed under the **MIT License**. For more details, please refer to the [LICENSE](./LICENSE) page.
