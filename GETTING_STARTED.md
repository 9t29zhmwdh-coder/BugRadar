# Getting Started with BugRadar

This guide walks you through setting up and running BugRadar from scratch, even if you have never used a terminal or built a Rust/Tauri app before. BugRadar runs on **Windows, Linux, and macOS**.

---

## Windows

### 1. Open a terminal

Right-click the Start button and choose **Terminal** (or **Windows PowerShell** on older versions of Windows).

### 2. Check prerequisites

Run each of these commands one by one:

```powershell
rustc --version
cargo --version
node --version
cargo tauri --version
```

If any of them prints something like `'rustc' is not recognized as an internal or external command`, that tool is missing or not on your PATH. Install what's needed:

- **Rust / Cargo**: install via [rustup.rs](https://rustup.rs) (run the installer, then restart your terminal)
- **Node.js**: install via [nodejs.org](https://nodejs.org) (LTS version recommended)
- **Tauri CLI**: once Rust/Cargo is installed, run `cargo install tauri-cli`

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [BugRadar GitHub page](https://github.com/9t29zhmwdh-coder/BugRadar)
2. Click the green **Code** button → **Download ZIP**
3. Extract the ZIP file somewhere convenient, e.g. `C:\Projekte\BugRadar-repo`

**Alternative (if you have git):**
```powershell
git clone https://github.com/9t29zhmwdh-coder/BugRadar.git
cd BugRadar
```

### 4. Build and run the desktop app

```powershell
cd frontend
npm install
cd ..
cargo tauri dev
```

The first run takes a while as Rust compiles all dependencies. Once it's done, a native BugRadar window should open, ready to point at a log file or Docker container.

<!-- TODO: Screenshot -->

### 5. Optional: CLI only

If you just want the command-line tool without the desktop UI:

```powershell
cargo install --path crates/br-cli
bugradar inspect C:\path\to\config.yaml
bugradar metrics
bugradar incidents --open
```

---

## Linux

### 1. Open a terminal

This depends on your desktop environment. Common shortcuts: **Ctrl+Alt+T**, or look for "Terminal" in your application menu.

### 2. Check prerequisites

```bash
rustc --version
cargo --version
node --version
cargo tauri --version
```

If you see `command not found` for any of these, install the missing piece:

- **Rust / Cargo**: install via [rustup.rs](https://rustup.rs)
- **Node.js**: install via [nodejs.org](https://nodejs.org) or your distro's package manager
- **Tauri CLI**: `cargo install tauri-cli`

Tauri apps on Linux also need WebKitGTK and a few system libraries — see Troubleshooting below if the build fails with missing package errors.

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [BugRadar GitHub page](https://github.com/9t29zhmwdh-coder/BugRadar)
2. Click the green **Code** button → **Download ZIP**
3. Extract it, e.g. into `~/Projekte/BugRadar-repo`

**Alternative (if you have git):**
```bash
git clone https://github.com/9t29zhmwdh-coder/BugRadar.git
cd BugRadar
```

### 4. Build and run the desktop app

```bash
cd frontend
npm install
cd ..
cargo tauri dev
```

A native BugRadar window should open once the build finishes.

### 5. Optional: CLI only

```bash
cargo install --path crates/br-cli
bugradar inspect /etc/nginx/nginx.conf
bugradar metrics
bugradar incidents --open
```

---

## macOS

### 1. Open a terminal

Press **Cmd+Space** to open Spotlight, type "Terminal", and press Enter.

### 2. Check prerequisites

```bash
rustc --version
cargo --version
node --version
cargo tauri --version
```

If you get a `command not found` error, that tool needs to be installed:

- **Rust / Cargo**: install via [rustup.rs](https://rustup.rs)
- **Node.js**: install via [nodejs.org](https://nodejs.org)
- **Tauri CLI**: `cargo install tauri-cli`

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [BugRadar GitHub page](https://github.com/9t29zhmwdh-coder/BugRadar)
2. Click the green **Code** button → **Download ZIP**
3. Extract it, e.g. into `~/Projekte/BugRadar-repo`

**Alternative (if you have git):**
```bash
git clone https://github.com/9t29zhmwdh-coder/BugRadar.git
cd BugRadar
```

### 4. Build and run the desktop app

```bash
cd frontend
npm install
cd ..
cargo tauri dev
```

A native BugRadar window should open once everything has compiled and installed.

### 5. Optional: CLI only

```bash
cargo install --path crates/br-cli
bugradar inspect /etc/nginx/nginx.conf
bugradar metrics
bugradar incidents --open
```

---

## Optional: Enable AI root-cause analysis

BugRadar can explain incidents using AI, but this is opt-in:

- **Claude (default)**: get an [Anthropic API key](https://console.anthropic.com/) and add it in **Settings**; it is stored in your OS keychain
- **Ollama (local, private)**: install [Ollama](https://ollama.ai), run `ollama pull llama3.2`, then set the host/model in **Settings**

Click "Run AI Analysis" on any incident to trigger it — it does not run automatically.

---

### Troubleshooting

| Issue | Cause | Fix |
|---|---|---|
| `'cargo' is not recognized` / `command not found: cargo` | Rust is not installed or not on PATH | Install via [rustup.rs](https://rustup.rs), then restart your terminal |
| `'npm' is not recognized` / `command not found: npm` | Node.js is not installed or not on PATH | Install via [nodejs.org](https://nodejs.org), then restart your terminal |
| PowerShell blocks a `.ps1` script with "running scripts is disabled on this system" | Windows execution policy restricts script execution | Run PowerShell as Administrator and execute `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`, then retry |
| Build fails on Windows with linker errors mentioning `link.exe` or MSVC | Missing C++ build tools required by Rust's MSVC toolchain | Install "Desktop development with C++" via the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) installer |
| `cargo tauri dev` fails on Linux with errors about `webkit2gtk` or `javascriptcoregtk` | Missing WebKitGTK system dependencies | On Debian/Ubuntu: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev` |
