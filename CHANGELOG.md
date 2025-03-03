# Changelog

## [Unreleased]

## [2.13.0-alpha] - 2025-02-20

### Added
- Subcommands to insert an item as the prior or next sibling of a node.
### Changed
- Updated app directory name to match app name.

## [2.12.0-alpha] - 2025-02-19

### Added
- **Insert** command to insert an item as the parent or child of a node.

## [2.11.0-alpha] - 2025-02-13

### Added
- **Raise** command to move a node's siblings to be its children.
- **Flatten** command to move a node's children to be its siblings.

## [2.10.0-alpha] - 2025-01-20

### Added
- **Promote** command to move a subtree up a level.
- **Demote** command to move a subtree down a level.

## [2.9.0-alpha] - 2025-01-13

### Removed
- **Compare** command.

## [2.8.0-alpha] - 2025-01-13

### Added
- Allow multiple consecutive moves.

### Changed
- Keep item selected when done moving.

## [2.7.0-alpha] - 2025-01-11

### Added
- **Move** command for moving a selected subtree within the forest.

## [2.6.0-alpha] - 2025-01-07

### Added
- Show indices for filenames.
- Enter digits to jump to filename indices.

## [2.5.1-alpha] - 2025-01-05

### Added
- Top and bottom indicators to show when scrolling is possible.

## [2.5.0-alpha] - 2025-01-03

### Added
- Scrolling ensures the selected filename stays visible.

## [2.4.0-alpha] - 2025-01-01

### Added
- **Rename** command to rename files.

## [2.3.0-alpha] - 2024-12-30

### Added
- Confirmation screen for messages and actions like deleting items/files.

## [2.2.0-alpha] - 2024-12-14

### Added
- **Load** command to return to file selection screen.

## [2.1.0-alpha] - 2024-12-07

### Added
- **Delete** command to delete files during file selection.

## [2.0.2-alpha] - 2024-12-06

### Fixed
- Prevent crash when attempting to save invalid filename.

## [2.0.1-alpha] - 2024-12-03

### Fixed
- Prevent overwriting existing files by enforcing unique filenames.

### Added
- Display reason for invalid input in status bar.

## [2.0.0-alpha] - 2024-11-24

### Added
- Support for saving multiple files.
- Display load screen on launch to allow file selection.
- Prompt the user to enter a filename when quitting an unsaved project.

## [1.3.0-alpha] - 2024-11-07

### Changed
- Hide indices after an item has been selected.

## [1.2.0-alpha] - 2024-11-05

### Added
- Skip save query if no changes were made.

## [1.1.0-alpha] - 2024-11-04

### Added
- Prompt the user to choose whether to save changes before quitting.

## [1.0.0-alpha] - 2024-10-27

### Changed
- Store data file in OS data directory.

### Security
- Lock data file while app is running, preventing overwriting saved data.
- Set data file to read-only.

## [0.1.0-alpha] - 2024-10-25

### Added
- Save and load binary data to/from fixed local file.
- General commands:
  - **Insert**: Add a new item.
  - **Select**: Select an item to apply a targeted command to.
  - **Compare**: Choose one item of a pair to promote over the other.
  - **Quit**: Quit after automatically saving.
- Targeted commands:
  - **Edit**: Modify item text.
  - **Delete**: Delete an item.

[Unreleased]: https://github.com/darthkeith/sieve-selector/compare/v2.13.0-alpha...HEAD
[2.13.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.12.0-alpha...v2.13.0-alpha
[2.12.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.11.0-alpha...v2.12.0-alpha
[2.11.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.10.0-alpha...v2.11.0-alpha
[2.10.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.9.0-alpha...v2.10.0-alpha
[2.9.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.8.0-alpha...v2.9.0-alpha
[2.8.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.7.0-alpha...v2.8.0-alpha
[2.7.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.6.0-alpha...v2.7.0-alpha
[2.6.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.5.1-alpha...v2.6.0-alpha
[2.5.1-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.5.0-alpha...v2.5.1-alpha
[2.5.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.4.0-alpha...v2.5.0-alpha
[2.4.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.3.0-alpha...v2.4.0-alpha
[2.3.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.2.0-alpha...v2.3.0-alpha
[2.2.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.1.0-alpha...v2.2.0-alpha
[2.1.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.0.2-alpha...v2.1.0-alpha
[2.0.2-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.0.1-alpha...v2.0.2-alpha
[2.0.1-alpha]: https://github.com/darthkeith/sieve-selector/compare/v2.0.0-alpha...v2.0.1-alpha
[2.0.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v1.3.0-alpha...v2.0.0-alpha
[1.3.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v1.2.0-alpha...v1.3.0-alpha
[1.2.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v1.1.0-alpha...v1.2.0-alpha
[1.1.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v1.0.0-alpha...v1.1.0-alpha
[1.0.0-alpha]: https://github.com/darthkeith/sieve-selector/compare/v0.1.0-alpha...v1.0.0-alpha
[0.1.0-alpha]: https://github.com/darthkeith/sieve-selector/releases/tag/v0.1.0-alpha

