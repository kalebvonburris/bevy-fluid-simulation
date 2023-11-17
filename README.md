# Bevy Fluid Simulation

A fluid simulation written in Rust using the [Bevy](https://bevyengine.org/) game engine.

## Running

### Compiled Binaries

Download the relevant binary from the [latest release](https://github.com/kalebvonburris/bevy-fluid-simulation/releases/latest) and run the given executable.

### Compiling Locally

#### Requirements

##### Rust

Minimum version: 1.75.0 Nightly

It's recommended to install Rust via [Rustup](https://rustup.rs/).

For Bevy-specific requirements, see the [setup section of the Bevy book](https://bevyengine.org/learn/book/getting-started/setup/).

##### Running locally

To run this project locally, clone this repository:

```bash
git clone https://github.com/kalebvonburris/bevy-fluid-simulation
```

`cd` into `bevy-fluid-simulation` and execute the following command:

```bash
cargo run --release
```

Note: *Bevy is an entire game engine. Thus, the build generated will take several minutes (for my 8-core AMD rig, 3-4 minutes) and create several gigabytes of extra data (3-4GB).*
