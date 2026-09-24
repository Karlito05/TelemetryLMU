<p align="center">
  <img
src="https://github.com/Karlito05/TelemetryLMU/blob/egui/public/icons/Logo.svg" width="128" alt="Our Logo"/>
</p>

<h1 align="center">Telemetry LMU</h1>

<p align="center">
  <strong>The ultimate telemetry visualization
tool for Le Mans Ultimate.</strong>
</p>

<p align="center">
  <img alt="GitHub License" src="https://img.shields.io/github/license/Karlito05/TelemetryLMU">
  <img
src="https://img.shields.io/github/stars/Karlito05/TelemetryLMU" alt="Stars">
  <img
src="https://img.shields.io/github/issues/Karlito05/TelemetryLMU" alt="Issues">
</p>

## 📺 Demo

https://youtu.be/aW0ga2AiyTQ

<img src="https://github.com/Karlito05/TelemetryLMU/blob/egui/Demo.png" />

## 🚀 Quick Start

### Windows

1. Navigate to the [Releases](https://github.com/Karlito05/TelemetryLMU/releases).
2. Download and run the latest `.exe` installer.

### Linux

1. **Prerequisite:** Install [SIMple Bridge](https://github.com/Karlito05/SIMple-Bridge)
2. Download the Arch Linux or Debian package from our [Releases](https://github.com/Karlito05/TelemetryLMU/releases).

## ✨ Features

### Telemetry

- **Customizable Layouts:** Tailor the UI to your preference.
- **Graphing:** Choose from multiple graph types to suit your needs.
- **Graph Customization:** Customize the graph color, the number of grid lines and more to suit your needs.

### Car Info

- **Detailed info:** See detailed information about your car at a glance.
- **Automatic detection:** Automatically detects your car and shows precise info.

### Map

- **Accelerated learning:** See your line against a reference and see where you made mistakes.
- **Inputs at a glance:** Shows your and the reference's inputs at a glance.
- **Delta:** Shows the time delta between you and your reference in real time.

## 🔨 Building

### Prerequisites

- Rust **Nightly**
- Docker (Optional - for packaging)

### How to build

1. Clone this repository with `git clone https://github.com/Karlito05/TelemetryLMU.git`
2. Change directory to be in the root of the project with `cd ./TelemetryLMU`
3. Build with `cargo build` or directly run with `cargo run`

4. Optionally you can make packages via running the `package_all.sh` in the root of the project.

## ⚙️ How it works

This app is made fully in rust! On the frontend side of things I am using [egui](https://github.com/emilk/egui) although almost all of the elements seen are custom made via the painter functionality.

On the backend side of things stuff gets more interesting. A thread (separate to the UI) is running a "telemetry provider" which records and stores all of the needed telemetry data. This means that the UI doesn't have to refresh when the app is minimized or when pages that don't need to update constantly (like the map page). The logger is also very light weight on the CPU side of things. It takes up a lot of RAM (around 200Mb) due to the amounts of data it is working with. However doing this this way ensures a smoother experience for the user!

## 📚 Credits

- This project relies heavily on the [egui](https://github.com/emilk/egui) library.
- Thanks to [Thrillonek](https://github.com/Thrillonek) for helping with the UI design.

---

<p align="center">Built for the Le Mans Ultimate community
🏎️</p>
