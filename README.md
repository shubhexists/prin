# Prin 🚀

A lightweight and user-friendly CLI reverse proxy tool built in Rust.

## Features ✨

- Simple and intuitive command-line interface
- Easy route management with interactive prompts
- Support for multiple proxy routes
- Persistent configuration across sessions

## Installation 🔧

```bash
cargo install prin
```

## Usage 💻

Prin offers two main commands:

### Start the Proxy Server

```bash
prin start [--port <PORT>]
```

Options:

- `--port, -p`: Specify the port to run the proxy server on (default: 8000)

### Configure Routes

```bash
# Add a new route
prin config add

# Edit an existing route
prin config edit

# Delete a route
prin config delete
```

## Configuration Storage 📁

Prin stores its configuration in JSON format at:

- Linux/macOS: `~/.config/prin/config.json`
- Windows: `%APPDATA%\prin\config.json`

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author ✍️

[shubhexists](https://github.com/shubhexists)
