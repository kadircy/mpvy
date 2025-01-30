# mpvy

**MPVY** is an open-source, lightweight, and terminal-based YouTube audio player, developed using the power of Rust programming language. Designed to offer a fast and customizable listening experience, MPVY leverages the MPV media player to seamlessly stream YouTube content as audio, supporting a variety of media formats. Whether you're an audiophile or a user seeking minimalistic, efficient tools, MPVY provides a user-friendly solution for enjoying YouTube audio directly from the terminal.

## Key Features

- **Lightweight & Efficient**: Built for performance, MPVY is designed to use minimal resources while delivering high-quality audio.
- **Terminal-based**: Operates entirely in the terminal, perfect for users who prefer a text-based interface or minimalistic setups.
- **Customizable**: Fully configurable to suit individual preferences, from audio quality to playlist management.
- **Supports CAVA Visualizer**: Add a console-based audio visualizer (CAVA) to enhance your listening experience.
- **Playlist Management**: Save and load playlists to effortlessly listen to your favorite tracks without needing to input queries repeatedly.
- **Easy Integration with MPV & yt-dlp**: By utilizing MPV and yt-dlp, MPVY ensures a smooth and reliable audio playback experience.

## Installation

To run **MPVY**, you need to install `MPV`, `yt-dlp`, and then build MPVY from source.

### Prerequisites

1. **Install MPV**: Make sure MPV is installed on your system. If you haven't already, refer to [MPV’s installation guide](https://mpv.io/).
2. **Install yt-dlp**: **yt-dlp** is a powerful downloader and extractor that MPVY uses for downloading YouTube audio. Install it by following the instructions on the [yt-dlp GitHub page](https://github.com/yt-dlp/yt-dlp).

### Building MPVY from Source

1. **Install Rust and Cargo**: Ensure Rust and Cargo are installed on your system. If you haven't installed them yet, visit [Rust’s installation page](https://www.rust-lang.org/tools/install).
   
2. **Clone the Repository**:
   ```
   git clone https://github.com/kadircy/mpvy
   ``` 

3. **Navigate to the Project Directory**:
   ```
   cd mpvy
   ```

4. **Build the Release Binary**:
   ```
   cargo build --release
   ```

5. **Run MPVY**: Now that you've compiled the binary, you can run MPVY with:
   ```
   ./target/release/mpvy
   ```

### Running MPVY

Simply run `mpvy` without any arguments, and the program will prompt you for a YouTube search query. You can enter multiple queries separated by commas. MPVY will then search, download, and play the audio for you using MPV. You can also interact with playback via IPC for additional control.

## Usage

### Basic Search and Playback

1. Launch the terminal and run:
   ```
   mpvy
   ```

2. Enter one or more YouTube search queries, separated by commas. MPVY will:
   - Search for the videos on YouTube.
   - Download them to your local machine (if they aren't already cached).
   - Fetch the audio using yt-dlp and launch MPV to play it.

Once the audio is playing, you can enjoy it across your system as MPV streams the content.

### Playlist Management

MPVY supports creating and playing local playlists. This feature allows you to curate and enjoy your favorite tracks in sequence, without re-entering search queries each time.

- **Save a Playlist**: Use the `--save-playlist <name>` option to save a playlist of your audio search queries:
   ```
   mpvy --save-playlist my_playlist
   ```
   This will store your queries in `$XDG_CONFIG_HOME/mpvy/playlist/my_playlist`.

- **Play a Saved Playlist**: To play a saved playlist, simply use:
   ```
   mpvy --playlist my_playlist
   ```

### Visualizer Support (CAVA)

For a more immersive experience, **MPVY** supports CAVA, a console-based audio visualizer. If you have CAVA installed, you can use the `--cava` flag to launch it automatically while your audio is playing:
   ```
   mpvy --cava
   ```
Once the audio finishes, CAVA will automatically close.

## Configuration

### Configuration File

MPVY uses a configuration file for managing various settings. The config file is located at `$XDG_CONFIG_HOME/mpvy/config.toml` (typically `~/.config/mpvy/config.toml`). Below are some key configurable options:

- **`max_file_count`**: Set the maximum number of audio files to save on your system.
   - Default: `15`
   - Example:
     ```
     max_file_count = 15
     ```

- **`audio_quality`**: Define the audio quality for downloads (lower values provide higher quality).
   - Default: `0` (best quality).
   - Example:
     ```
     audio_quality = 0
     ```

- **`concurrent_fragments`**: Set the number of concurrent fragments for downloading audio using `yt-dlp`.
   - Default: `4`
   - Example:
     ```
     concurrent_fragments = 4
     ```

### IPC (Inter-Process Communication)

MPVY allows interaction with MPV instances using IPC sockets, making it possible to control the audio playback programmatically. By default, MPVY sets the IPC socket path to:
   ```
   $XDG_CONFIG_HOME/mpv/socket
   ```
To control audio playback, simply interact with this socket. You can use MPV's IPC commands to control actions like play, pause, volume adjustment, etc.

### Logs

MPVY stores logs for both the MPVY program and the MPV media player at the following location:
   ```
   $XDG_CONFIG_HOME/mpvy/log
   ```
Please note that the logs are overwritten with each command execution, so previous logs will be deleted each time the application is run.

## Contributing

Contributions to **MPVY** are welcome and highly appreciated! If you'd like to contribute, please follow the steps below:

1. **Fork the repository**.
2. **Create a new branch** for your changes.
3. **Make your changes**, ensuring that you follow the existing coding style. Run `cargo fmt` to format your code.
4. **Run tests** (if applicable).
5. **Submit a pull request** with a clear description of your changes.

If you encounter bugs or have feature suggestions, feel free to open an issue. We appreciate all contributions and feedback!

## Changelog

For a detailed list of changes, updates, and version history, please refer to the [CHANGELOG](./CHANGELOG.md) page.

## License

This project is licensed under the **MIT License**. For more details, please refer to the [LICENSE](./LICENSE) page.
