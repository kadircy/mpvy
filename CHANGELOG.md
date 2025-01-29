# Changelog

## [Unreleased]

## [0.2.1] - 2025-01-29
### Added

- New feature: Support for **playlists** to play audios with a local list.
- Introduced **cava** integration to display an audio visualizer during playback (use the `--cava` flag).
- Implemented automatic logging of mpvy and **mpv** activities to `$XDG_CONFIG_HOME/mpvy/log/mpvy.log` and `$XDG_CONFIG_HOME/mpvy/log/mpv.log`.
- New feature: Support for **yt-dlp** to fetch YouTube video information and audio downloads.
- Config file planned for future release at `$XDG_CONFIG_HOME/mpvy/config.toml`.

### Changed

- Refactored the way of getting info about audio. Removed unused informations.
- Refined the way mpv processes videos: Added IPC support for better interaction with the player.
- Updated video download logic to check if files are already downloaded before starting the download process.
- Logging improvements with more detailed **error**, **info**, and **warning** messages.

### Fixed

- Fixed an issue where **mpvy** would fail when empty input given.
- Fixed an issue where **yt-dlp** would fail silently when video info couldn't be fetched.
- Resolved the problem of incorrect directory paths when fetching or saving media files.
- Improved handling of errors in spawning external processes like **mpv** and **yt-dlp**.

## [0.1.0] - 2025-01-27
### Added

- Initial release with support for searching and playing YouTube audio via terminal.
- Implemented basic logging functionality for tracking program activities.
- The ability to clear the terminal and ask for user input on search queries.
- Integration with mpv for audio playback and **yt-dlp** for fetching and downloading video audio.