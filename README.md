# Mac-OLS

![Build](https://github.com/MykeHaunt/Mac-OLS/actions/workflows/conda-package.yml/badge.svg)
![Python](https://img.shields.io/badge/python-3.7%2B-blue.svg)
![License](https://img.shields.io/github/license/MykeHaunt/Mac-OLS)
![Status](https://img.shields.io/badge/status-Beta-blue.svg)
![Last Commit](https://img.shields.io/github/last-commit/MykeHaunt/Mac-OLS)
![Contributors](https://img.shields.io/github/contributors/MykeHaunt/Mac-OLS)
![Issues](https://img.shields.io/github/issues/MykeHaunt/Mac-OLS)
![Stars](https://img.shields.io/github/stars/MykeHaunt/Mac-OLS?style=social)
![Forks](https://img.shields.io/github/forks/MykeHaunt/Mac-OLS?style=social)

> **WORK IN PROGRESS BY: H. Pandit**  

A **native, open-source** alternative to WinOLS, designed specifically for macOS on Apple Silicon. This modular toolkit provides **full ECU tuning capabilities**, including binary file editing, map detection, checksum correction, and Damos/A2L parsing—all with a modern, cross-platform core in **Rust** and a native **SwiftUI** GUI.  

### **Key Features**  
✅ **Native Apple Silicon Support** – Optimized for macOS performance  
✅ **WinOLS Feature Parity** – Map detection, hex editing, checksum correction  
✅ **Modular Plugin System** – Extend functionality with `.dylib` plugins  
✅ **Dual Interface** – CLI (`clap`) for scripting + SwiftUI GUI for intuitive use  
✅ **Cross-Platform Core** – Rust engine with C-ABI bindings for future Windows/Linux ports  

### **Tech Stack**  
- **Core**: Rust (safe, high-performance binary operations)  
- **CLI**: Rust + `clap` (scriptable workflows)  
- **GUI**: SwiftUI (native macOS interface)  
- **Interop**: `cbindgen` for Rust→Swift bridging  
- **CI/CD**: GitHub Actions (automated builds/tests)  

### **Use Cases**  
- **Tuners**: Edit ECU binaries natively on macOS  
- **Developers**: Build plugins for custom ECUs/checksums  
- **Enthusiasts**: Learn reverse engineering with open tools  

```bash
# Quick Start (CLI)
cargo install --path cli
ecutil-cli detect firmware.bin
```

**License**: Apache 2.0 | **Contributions Welcome!**  

---

**Why This Project?**  
Most ECU tools are Windows-only (x86) or proprietary. This suite fills the gap with:  
- 🍏 **No Wine/Emulation** – True native performance  
- 🔓 **Transparent Algorithms** – Open heuristics for map detection  
- 🧩 **Future-Proof** – Plugin API for emerging ECU standards  

Target audience: **Automotive tuners, embedded devs, and FOSS enthusiasts**.  

*(Repository includes: Rust core, CLI, SwiftUI GUI skeleton, CI, and docs.)*
