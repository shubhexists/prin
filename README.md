# Prin 🚀

A lightweight and user-friendly CLI reverse proxy tool built in Rust.

## Features ✨

- Simple and intuitive command-line interface
- Easy route management with interactive prompts
- Support for multiple proxy routes
- Persistent configuration across sessions

## Installation 🔧

```bash
# Clone the repository
git clone https://github.com/shubhexists/prin.git

# Navigate to the project directory
cd prin

# Build and install using Cargo
cargo install --path .
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

## Example Configuration 🛠️

```bash
# Add a new route
$ prin config add
=== Adding New Route ===
🔗 Enter route prefix: /api
🎯 Enter target URL: http://localhost:3000
⚡ Add route: /api → http://localhost:3000? Y/n
✅ Route added: /api → http://localhost:3000

# Start the server
$ prin start
✅ Loaded configuration.
🔗 Configured Routes:
✅ /api → http://localhost:3000
🚀 Running server on 127.0.0.1:8000
```

## Configuration Storage 📁

Prin stores its configuration in JSON format at:

- Linux/macOS: `~/.config/prin/config.json`
- Windows: `%APPDATA%\prin\config.json`

## Dependencies 📦

- clap: Command line argument parsing
- colored: Terminal colors and styling
- dialoguer: Interactive CLI prompts
- hyper: HTTP server implementation
- serde: Serialization/deserialization for configuration
- tokio: Async runtime
- dirs: Cross-platform config directory detection

## Contributing 🤝

Contributions are welcome! Please feel free to submit pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author ✍️

[shubhexists](https://github.com/shubhexists)
