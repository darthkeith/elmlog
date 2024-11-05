# Changelog

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

