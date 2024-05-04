# AutoIPChanger

AutoIPChanger is a Rust script designed to automate the process of installing Tor, configuring system-wide proxy settings, and monitoring IP changes using Tor's SOCKS proxy. This script enhances anonymity by continuously monitoring IP changes through Tor's circuit path display.

## Features

- Installs Tor via `sudo apt install tor`.
- Configures system-wide proxy settings to use Tor SOCKS proxy.
- Periodically checks for IP changes every 50 seconds.
- Displays Tor circuit path along with the IP address for enhanced anonymity.

## Usage

1. Clone the repository:
   ```
   git clone https://github.com/your-username/autoIPChanger.git
   ```

2. Navigate to the project directory:
   ```
   cd autoIPChanger
   ```

3. Run the script using Cargo:
   ```
   cargo run
   ```

4. The script will install Tor, configure the proxy settings, and start monitoring IP changes through Tor's SOCKS proxy.

## Requirements

- Rust
- Tor

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.