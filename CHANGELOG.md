# Changelog

## [Unreleased]

## [0.22.0] - 2025-02-20

### Added
- Subcommands to insert an item as the prior or next sibling of a node.
### Changed
- Updated app directory name to match app name.

## 0.21.0 - 2025-02-19

### Added
- **Insert** command to insert an item as the parent or child of a node.

## 0.20.0 - 2025-02-13

### Added
- **Raise** command to move a node's siblings to be its children.
- **Flatten** command to move a node's children to be its siblings.

## 0.19.0 - 2025-01-20

### Added
- **Promote** command to move a subtree up a level.
- **Demote** command to move a subtree down a level.

## 0.18.0 - 2025-01-13

### Removed
- **Compare** command.

## 0.17.0 - 2025-01-13

### Added
- Allow multiple consecutive moves.

### Changed
- Keep item selected when done moving.

## 0.16.0 - 2025-01-11

### Added
- **Move** command for moving a selected subtree within the forest.

## 0.15.0 - 2025-01-07

### Added
- Show indices for filenames.
- Enter digits to jump to filename indices.

## 0.14.0 - 2025-01-05

### Added
- Top and bottom indicators to show when scrolling is possible.

## 0.13.0 - 2025-01-03

### Added
- Scrolling ensures the selected filename stays visible.

## 0.12.0 - 2025-01-01

### Added
- **Rename** command to rename files.

## 0.11.0 - 2024-12-30

### Added
- Confirmation screen for messages and actions like deleting items/files.

## 0.10.0 - 2024-12-14

### Added
- **Load** command to return to file selection screen.

## 0.9.0 - 2024-12-07

### Added
- **Delete** command to delete files during file selection.

## 0.8.0 - 2024-12-06

### Fixed
- Prevent crash when attempting to save invalid filename.

## 0.7.0 - 2024-12-03

### Fixed
- Prevent overwriting existing files by enforcing unique filenames.

### Added
- Display reason for invalid input in status bar.

## 0.6.0 - 2024-11-24

### Added
- Support for saving multiple files.
- Display load screen on launch to allow file selection.
- Prompt the user to enter a filename when quitting an unsaved project.

## 0.5.0 - 2024-11-07

### Changed
- Hide indices after an item has been selected.

## 0.4.0 - 2024-11-05

### Added
- Skip save query if no changes were made.

## 0.3.0 - 2024-11-04

### Added
- Prompt the user to choose whether to save changes before quitting.

## 0.2.0 - 2024-10-27

### Changed
- Store data file in OS data directory.

### Security
- Lock data file while app is running, preventing overwriting saved data.
- Set data file to read-only.

## 0.1.0 - 2024-10-25

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

[Unreleased]: https://github.com/darthkeith/sieve-selector/compare/v0.22.0...HEAD
[0.22.0]: https://github.com/darthkeith/sieve-selector/compare/v0.21.0...v0.22.0

