# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2025-04-19

## Added

- Add key bindings:
  - TAB: Switch to next tool
  - BACKTAB: Switch to prev tool
  - p: Switch to PICK tool
  - d: Switch to DRAW tool
  - f: Switch to FILL tool
  - e: Switch to ERASE tool
  - s: Switch to SELECT tool
  - m: Switch to MOVE tool
  - Ctrl-z: undo
  - Ctrl-y: redo
  - <: Switch to prev frame
  - >: Switch to next frame

## [0.8.0] - 2025-04-06

### Added

- Add an `APNG` setting to switch the export image format between APNG and sprite sheet (a plain PNG file with animation frames lying adjacently)

### Fixed

- Remove unnecessary debug print

## [0.7.0] - 2024-08-06

### Removed

- Remove sub-tool widgets for drawing and selecting
  - The following tools are deleted:
    - Draw line
    - Draw rectangle
    - Draw circle
    - Select rectangle
    - Import PNG image

## [0.6.0] - 2024-03-23

### Fixed

- Fix a bug that a non-move drawing could increase undo counter to the maximum value
- Don't cache fetched workspace files

### Added

- Make background color configurable
- Add auto generated color palette block to the color selector window
- Add touch gesture support
  - If the `gesture` setting is enabled, the following gestures will become available:
    - Tap: switch to picker tool
    - Two-finger tap: switch to selection tool
    - Three-finger tap: switch to bucket tool
    - Swipe: camera move
    - Two-finger horizontal swipe: undo / redo
    - Pinch: zoom in / out
- Add frame size double / halve buttons
- Add a button to set the frame size to the tool size
- Add a button to adjust opacity of the selected pixels
- Show prev / next frame buttons if animation is enabled
- Automatically determine the number of animation frames
- Automatically determine the number of layers
- Add a setting to enable frame previews in silhouette mode
- Add non square pixel size support
- Add `orfail` crate to the dependencies

### Removed

- Remove select-bucket tool
- Remove move-tool window
- Remove frame count setting
- Remove layer count setting
- Remove max undos setting
- Remove import-from-clipboard feature
- Remove finger mode
- Remove button long press feature
- Remove `pixcil_windows` crate to reduce maintenance costs

### Changed

- Change the default selection tool to lasso
- Move bucket tool from draw sub tools to main tools
- Switch to the erase-tool if picker-tool selects a pixel doesn't have a color
- Rename config item name: s/PIXEL SIZE/TOOL SIZE/
- Allow non-drawn selected region to be draggable during manipulation
- Merge frame width / height settings into frame size setting
- Don't save undo buffer in the image file
- Change default pixel size to 1
- Change PWA display mode from "minimal-ui" to "standalone"
- Update pagurus version from v0.6 to v0.7
- Update libflate version from v1 to v2

## [0.5.0] - 2023-05-31

### Added

- Add created time and updated time attributes
- Add `load` query string parameter to specify a URL of a PNG image to load (Web)
- Add vibration when drawing / erasing / selecting actions are completed
- Add bucket selecting tool
- Add preview scale setting
- Add import-image-from-clipboard feature
- Add finger friendly drawing mode

### Changed

- Set `CanvasRenderingContext2D.imageSmoothingEnabled` to `true`

## [0.4.0] - 2023-02-18

### Added

- Ellipse drawing tool
- Small screen support (auto resize)
- PWA (Progressive Web Apps) support
- Make it possible to load PNG files using palette mode
- VSCode extension

### Fixed

- Don't let preview area consume mouse events for buttons
- Don't reset "FRAME PREVIEW" setting when opening settings dialog
- Ensure that preview size reflects loaded image size

## [0.3.0]

- [UPDATE] Support to import gray scale PNG files
- [FIX] Fix a bug that the program crashes when an HSV color slider reaches the max value and then the up button is pressed
- [CHANGE] Limit maximum FPS to 120 to eliminate too many redraws
- [Add] Use service worker to support offline mode
- [UPDATE] Remove green margin around the canvas (web)
- [ADD] Windows binary

## [0.2.0]

- [UPDATE] Skip storing a pixel instance if it's alpha is zero
- [FIX] Ensure imported image positions align with pixel size
- [UPDATE] Brighten unfocused text box background color
- [CHANGE] Remove erasing tool dialog as it's redundant with the selection-then-cut feature
- [UPDATE] Merge multiple redraw requests issued during an event handling
- [UPDATE] Change the pick-button into non-clickable state when it's selected
- [ADD] Add buttons to halve / double the pixel size setting
- [FIX] Consider pixel size when copying
- [ADD] Support flip and rotate operations

## [0.0.13] - 2023-05-31

- Add created time and updated time attributes

## [0.0.12] - 2023-05-27

### Added

- Add vibration when drawing / erasing / selecting actions are completed

### Fixed

- Fix preview bug when preview scale > 1 and frame count > 1

## 0.0.11

### Added

- Add bucket selecting tool

## 0.0.10

### Added

- Add "PREVIEW SCALE" option to settings window
- Add import-image-from-clipboard feature

## 0.0.9

### Added

- Add "FINGER MODE" switch to settings window

## 0.0.8

### Added

- Add ellipse drawing tool

## 0.0.6

### Added

- Support small screen (auto resize)

## 0.0.5

### Fixed

- Fix a bug that preview size doesn't reflect loaded image size

## 0.0.1

- Initial release
