# Progranome - Programmable Metronome
## Complete Project Documentation

---

## Table of Contents
1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Building & Setup](#building--setup)
4. [Core Components](#core-components)
5. [Module Documentation](#module-documentation)
6. [UI Structure](#ui-structure)
7. [Development Guide](#development-guide)

---

## Project Overview

**Progranome** is a programmable metronome application designed for Android devices. It allows musicians to create complex, customizable metronome patterns with multiple time signatures and tempos that can be played simultaneously or sequentially.

### Key Features
- **Multiple Metronome Units**: Support for multiple independent metronome tracks
- **Programmable Time Signatures**: Define custom numerator/denominator patterns
- **Variable Tempo Control**: Set tempo in BPM for each unit
- **Audio Synthesis**: Built-in sound generation with frequency-based tones
- **Android Integration**: Native Android app built with Rust

### Technology Stack
- **Language**: Rust (Edition 2024)
- **UI Framework**: Slint 1.15.1 with Android backend
- **Audio**: TinyAudio 2.0.0
- **Async Runtime**: async-std 1.13.2
- **Logging**: log 0.4.29

---

## Architecture

### Build System
The project uses Cargo as its package manager and includes a build script (`build.rs`) for Slint UI compilation.

```
progranome/
├── build.rs                 # Build script for Slint compilation
├── Cargo.toml              # Package manifest & dependencies
├── Cargo.lock              # Locked dependency versions
├── src/
│   ├── lib.rs              # Main library entry & Android integration
│   ├── metronome_unit.rs   # Core metronome unit structure
│   └── ui/                 # Slint UI definitions
└── target/                 # Build artifacts (debug & release)
```

### Android Configuration
- **Min SDK Version**: 26
- **Target SDK Version**: 33
- **Max SDK Version**: 29
- **Target Architecture**: aarch64-linux-android (ARM64)

---

## Building & Setup

### Prerequisites
- Rust 1.70+ (with `aarch64-linux-android` target)
- Android NDK (for Android builds)
- Android SDK (API 26-33)
- Cargo-APK tool

### Build Commands

**Debug Build for Android:**
```bash
cargo apk build --lib
```

**Release Build for Android:**
```bash
cargo apk build --lib --release
```

**Desktop/Testing Build:**
```bash
cargo build
```

The output APK is generated at:
```
target/debug/apk/progranome.apk
```

---

## Core Components

### 1. Main Library Entry (`lib.rs`)

The main Rust library handles:
- Android app initialization
- UI window creation and lifecycle
- Event handling (play button presses)
- Audio device management
- Timer logic for metronome ticking

**Key Functions:**
- `android_main()`: Entry point for Android app lifecycle
- `tick()`: Async audio generation function that produces sine wave tones

**Current State:**
- Most metronome logic is commented out (under development)
- Play button handler exists but needs activation
- Basic audio infrastructure is in place

### 2. Metronome Unit (`metronome_unit.rs`)

Represents a single metronome track with configurable parameters:

```rust
pub struct MetronomeUnit {
    numerator: u8,        // Top number of time signature
    denominator: u8,      // Bottom number of time signature
    tempo: u16,           // Tempo in BPM
    active: bool,         // Whether this unit is playing
}
```

**Example:** 4/4 time at 120 BPM would be:
- `numerator: 4`
- `denominator: 4`
- `tempo: 120`

---

## Module Documentation

### Audio System

**Audio Parameters:**
```rust
OutputDeviceParameters {
    channels_count: 2,           // Stereo audio
    sample_rate: 44100,          // 44.1 kHz sampling
    channel_sample_count: 4410,  // ~100ms buffer at 44.1 kHz
}
```

**Tone Generation:**
- Primary beat (beat 1): 800 Hz tone
- Secondary beats: 400 Hz tone
- Duration: 100 milliseconds
- Waveform: Sine wave

### UI Components

The UI is defined using Slint's declarative language across multiple files:

| File | Purpose |
|------|---------|
| `main.slint` | Root window layout |
| `logic.slint` | Application state & logic |
| `metronome_unit.slint` | Single unit UI component |
| `unit_panel.slint` | Panel for managing multiple units |
| `metronomePanel.slint` | Main control panel |
| `control_panel.slint` | Global controls (play/pause/stop) |
| `palette.slint` | Color scheme & theming |

---

## UI Structure

### Main Window Components

1. **Control Panel** - Global playback controls
   - Play button (currently implemented)
   - Pause button
   - Stop button

2. **Unit Panel** - Multiple metronome unit management
   - Add/remove units
   - Individual unit controls
   - Tempo adjustment
   - Time signature editing

3. **Metronome Display**
   - Current beat indicator
   - Visual feedback for timing
   - Tempo display

---

## Development Guide

### Adding a New Feature

#### 1. Implement Rust Logic (`lib.rs` or new module)
```rust
// Example: Add a feature module
mod my_feature;
use my_feature::MyFeature;
```

#### 2. Update UI Binding (`*.slint`)
```slint
// Link Rust code to UI
export component MyUI {
    // UI definition
}
```

#### 3. Handle Events
```rust
main.on_my_button_pressed(move || {
    // Handle button press
});
```

### Key Development Notes

- **Async Audio**: Audio operations use `async_std` for non-blocking operation
- **State Management**: Use Slint global properties for shared state
- **Memory Management**: Use `Rc<RefCell<T>>` for shared mutable state
- **Android Specifics**: Code gated with `#[cfg(target_os = "android")]`

### Currently Commented-Out Features

The play button handler contains extensive commented code for:
- Metronome timing loop
- Beat counting logic
- Audio tick generation
- UI state updates

This code needs to be:
1. Uncommented and debugged
2. Refactored into reusable functions
3. Integrated with the actual metronome logic

---

## Data Flow

```
┌─────────────────────────────────────────┐
│     Android Activity / MainWindow       │
└────────────┬────────────────────────────┘
             │
             ├─── on_play_button_pressed()
             │
         ┌───┴─────────────────────────────┐
         │  Timer (Repeated, 1-second)      │
         └───┬─────────────────────────────┘
             │
             ├─── Calculate beat (numerator-based)
             │
         ┌───┴─────────────────────────────┐
         │  Generate Audio (tick function)  │
         │  Frequency: 800Hz (beat 1) or   │
         │           400Hz (other beats)   │
         └─────────────────────────────────┘
```

---

## Next Steps & TODO

- [ ] Complete metronome tick logic (uncomment and refactor)
- [ ] Implement beat animation/visual feedback
- [ ] Add tempo tap detection
- [ ] Support for multiple simultaneous units
- [ ] Save/load metronome patterns
- [ ] Add settings screen
- [ ] Implement audio visualization
- [ ] Add volume control
- [ ] Support for different sound packs

---

## Troubleshooting

### APK Build Fails
- Ensure Android NDK is installed and in PATH
- Verify SDK versions match Cargo.toml settings
- Check that `aarch64-linux-android` target is installed: `rustup target add aarch64-linux-android`

### No Sound Output
- Verify `tinyaudio` backend is properly initialized
- Check device audio permissions in AndroidManifest.xml
- Ensure sample rate (44100 Hz) is supported by device

### UI Not Rendering
- Rebuild Slint modules: `cargo clean && cargo apk build --lib`
- Check Slint syntax in `.slint` files
- Verify all imports in `logic.slint`

---

## License & Credits

**Project**: Progranome - Programmable Metronome
**Version**: 0.1.0
**Status**: Early Development

---

*Generated on March 12, 2026 | For latest updates, see repository commits*
