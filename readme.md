# Recoil Control Script for Rainbow Six Siege

This project is a Rust-based recoil control script designed for Rainbow Six Siege. It provides a GUI for configuring recoil settings and simulates mouse movements to counteract weapon recoil during gameplay.

I have purposefully over-engineered some aspects since I am trying to learn Rust for work.

## Features

- **Recoil Control Logic**: Simulates mouse movement to counteract recoil when both left and right mouse buttons are pressed.
- **GUI Integration**: Built using the `eframe` crate, the GUI allows users to:
  - Toggle recoil control on/off.
  - Select operators and adjust their recoil strength.
  - Save and reset operator settings.
- **Error Handling**: Logs meaningful error messages for debugging.
- **Performance**: Optimized to reduce CPU usage and ensure responsiveness.

## Getting Started

### Prerequisites

- Rust programming language installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- Git is installed. You can download it from [git](https://git-scm.com/downloads)
- Windows operating system (required for compatibility & idek if Rainbow Six Siege is on other platforms).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/northaxosky/R6RustRecoilScript
   cd recoil_control
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## Usage

1. Launch the application.
2. Use the GUI to:
   - Toggle recoil control on/off.
   - Select an operator from the dropdown menu.
   - Adjust the recoil strength using the slider.
   - Save or reset operator settings.
3. During gameplay, aim down sights and shoot...

## File Structure

- `src/`
  - `main.rs`: Entry point of the application.
  - `gui.rs`: Handles the GUI logic.
  - `recoil.rs`: Contains the recoil control logic.
  - `operators.rs`: Manages operator data and file I/O.
- `operators.json`: Stores operator data, including default and current recoil strengths.

## Contributing

I have no idea how to use Rust please feel free.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments

- [rdev](https://crates.io/crates/rdev) for mouse event detection.
- [enigo](https://crates.io/crates/enigo) for simulating mouse movements.
- [eframe](https://crates.io/crates/eframe) for building the GUI.
