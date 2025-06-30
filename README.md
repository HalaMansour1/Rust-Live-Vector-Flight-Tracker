# SkyRadar - Live Aircraft Tracker

A Rust desktop application that displays live aircraft data in a radar-style interface.

## Features

- **Live Aircraft Data**: Pulls real-time aircraft positions from OpenSky Network API
- **Radar View**: Interactive map showing aircraft with directional arrows and altitude color coding
- **Location Setting**: Set your location or auto-detect via IP
- **Auto-refresh**: Updates aircraft positions every 30-60 seconds
- **Flight Details**: Click aircraft to see callsign, altitude, speed, and destination
- **Dark/Light Themes**: Toggle between themes
- **Cross-platform**: Works on Windows, macOS, and Linux

## How to Run

### Prerequisites
- Rust installed on your system

### Step 1: Clone and Build
```bash
git clone https://github.com/yourusername/Rust-Live-Vector-Flight-Tracker.git
cd Rust-Live-Vector-Flight-Tracker
cargo build --release
```

### Step 2: Run the Application
```bash
# Option 1: Run directly
./target/release/skyradar

# Option 2: Install globally and run from anywhere
cargo install --path .
skyradar
```

### Step 3: Configure (Optional)
- Set your location in the settings panel
- Add OpenSky API credentials for real data (optional - app works with mock data too)

That's it! The app will open in a native desktop window showing live aircraft around your location.
