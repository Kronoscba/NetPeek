
cat > README.md << 'EOF'
# 🚀 NetPeek

> **Fast and simple port scanner written in Rust with Python integration**\
> High-performance network reconnaissance tool for security testing and network exploration.

## ![Rust](https://img.shields.io/badge/Rust-1.80+-CE422B.svg) ![clap](https://img.shields.io/badge/clap-4.5+-blue.svg) ![Performance](https://img.shields.io/badge/Performance-High_Speed-green.svg) ![Security](https://img.shields.io/badge/Security-Network_Scanning-8A2BE2) ![License](https://img.shields.io/badge/License-MIT-green.svg)

## 🎯 Quick Start

\`\`\`bash
cargo build --release
./target/release/escaner_rapido <HOST> [OPTIONS]
\`\`\`

## ⚡ Features

- **Ultra-fast scanning**: Rust performance for rapid port enumeration
- **Multi-threaded**: Concurrent socket connections
- **Python integration**: Easy bindings for scripting
- **Simple CLI**: User-friendly command interface
- **Lightweight**: Minimal dependencies
- **Cross-platform**: Windows, macOS, Linux

---

## 📦 Installation

```
git clone https://github.com/Kronoscba/NetPeek.git
cd NetPeek

cargo build --release
cargo run --release -- <HOST> [PORTS]
```

> Requires **Rust 1.80+**

---

## 🔧 Usage

```
./target/release/escaner_rapido 192.168.1.1
./target/release/escaner_rapido example.com --ports 80,443,8080
./target/release/escaner_rapido 10.0.0.1 --ports 1-1000
./target/release/escaner_rapido --help
```

---

## 📊 Performance

- ~10K ports/second on modern hardware
- Async I/O for concurrent connections
- Minimal memory footprint

---

## ⚖️ Ethics

⚠️ **Only authorized security testing.**

---

## 📝 License

MIT — see [LICENSE](LICENSE).

## ⭐ Like it? Star it!


MIT License

Copyright (c) 2025 Kronoscba

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF

