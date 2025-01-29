# mpvy

https://github.com/user-attachments/assets/7544f0a6-da32-43aa-b4da-0eb10c2a0d2d

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
**mpvy** now supports local playlists, allowing you to play multiple audio tracks repeatedly without having to enter the query each time. To save a playlist, use the `--save-playlist <name>` argument and enter your video queries as usual. **mpvy** will store these queries in a file located at `$XDG_CONFIG_HOME/mpvy/playlist/<name>`. To play a saved playlist, simply use the `--playlist <name>` argument, and **mpvy** will handle playback seamlessly.

## Cava
**mpvy** has built-in support for displaying **cava** (a console-based audio visualizer) while playing audio. If you have **cava** installed on your system, you can use the `--cava` argument to automatically launch cava when the audio starts. Once all audio has finished playing, **cava** will be closed automatically.

## Configuration
Currently, configuration options are not available in **mpvy**. However, we are planning to introduce a configuration file in the future. The configuration file will be located at `$XDG_CONFIG_HOME/mpvy/config.toml` (typically `~/.config/mpvy/config.toml`). This feature will be added soon, allowing users to customize their **mpvy** experience more easily. Stay tuned for future updates!

## Logs
You can access both **mpvy** and **mpv** logs in the `$XDG_CONFIG_HOME/mpvy/log` directory. Please note that these logs are overwritten from scratch with every **mpvy** command, so previous logs are deleted each time.

## Contributing

Contributions to **mpvy** are welcome and appreciated! If you'd like to contribute, please fork the repository, make your changes, and submit a pull request. Before submitting, ensure your code adheres to the existing style (run `cargo fmt` and resolve any warnings) and includes any necessary tests. If you encounter bugs or have feature suggestions, feel free to open an issue. Thank you for your support!

## Changelog
For more details on updates and changes, please refer to the [CHANGELOG](./CHANGELOG.md) page.

## License
This project is licensed under the **MIT License**. For more details, please refer to the [LICENSE](./LICENSE) page.
