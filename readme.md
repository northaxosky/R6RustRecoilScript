# 🕹️ Recoil Control Script for Rainbow Six Siege

This project is a **Rust-based** recoil control script designed for **Rainbow Six Siege** 🎯  
It provides a **GUI** for configuring recoil settings and simulates mouse movements to counteract weapon recoil during gameplay 🖱️

> ⚠️ I have purposefully over-engineered some aspects since I am trying to **learn Rust** for work.

---

## ✨ Features

- 🎮 **Recoil Control Logic**: Simulates mouse movement to counteract recoil when both left and right mouse buttons are pressed.
- 🧩 **GUI Integration**: Built using the `eframe` crate, the GUI allows users to:
  - ✅ Toggle recoil control on/off  
  - 🎯 Select operators and adjust their recoil strength  
  - 💾 Save and 🔄 reset operator settings
- 🐛 **Error Handling**: Logs meaningful error messages for debugging.
- ⚡ **Performance**: Optimized to reduce CPU usage and ensure responsiveness.

---

## 🚀 Getting Started

### 📦 Prerequisites

- 🦀 [Rust](https://www.rust-lang.org) programming language installed  
- 🧰 [Git](https://git-scm.com) installed  
- 🪟 Windows OS (required for compatibility — and let's be real, is R6 even on anything else?)

---

### 🛠️ Installation

Clone the repository:

```bash
git clone https://github.com/northaxosky/R6RustRecoilScript
cd R6RustRecoilScript
```

Build the project:

```bash
cargo build --release
```

Run the application:

```bash
cargo run
```

---

## 🕹️ Usage

1. Launch the application 💻  
2. Use the GUI to:
   - 🔘 Toggle recoil control on/off
   - 👤 Select an operator from the dropdown
   - 🎚️ Adjust the recoil strength with the slider
   - 💾 Save or 🔄 reset operator settings
3. During gameplay:  
   🎯 Aim down sights and 🔫 shoot like a pro — recoil handled!

---

## 📁 File Structure

```
src/
├── main.rs        # 🚪 Entry point of the application
├── gui.rs         # 🖼️ Handles the GUI logic
├── recoil.rs      # 🎯 Recoil control logic
├── operators.rs   # 👥 Operator data and file I/O
operators.json     # 📄 Stores operator data
```

---

## 🤝 Contributing

I have no idea how to use Rust 🙃  
Please feel free to open issues, make pull requests, or just laugh at my code.

---

## 📜 License

📝 This project is licensed under the **MIT License**. See the `LICENSE` file for details.

---

## 🙌 Acknowledgments

- 🖱️ `rdev` for mouse event detection  
- 🎮 `enigo` for simulating mouse movements  
- 🧱 `eframe` for building the GUI
