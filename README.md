# Menu

![Rust CI](https://github.com/rileyhernandez/menu/workflows/Rust%20CI/badge.svg)

A Rust configuration management library for Caldo's kitchen automation devices.

## Overview

Menu is an internal library that provides configuration management and backend communication for Caldo's automated kitchen equipment. It supports two main device types:

- **Ichibu** (V1/V2): Automated food dispensing devices with conveyor motors, hatches, photo eyes, and dispensing mechanisms
- **Libra** (V0): Precision weighing scales with load cells and Phidget integration

## Features

### Device Management
- Device identification with model types and serial numbers
- Support for multiple device generations (IchibuV1, IchibuV2, LibraV0)
- Configuration serialization/deserialization with TOML format

### Backend Integration
- RESTful API communication with Caldo's cloud backend
- Authentication via bearer tokens
- Device registration, configuration retrieval, and updates
- Address management for devices
- Both synchronous and asynchronous API support

### Configuration Management
- Local TOML configuration file management
- CRUD operations for device configurations
- Batch configuration operations
- Configuration validation and error handling

### Hardware Integration
- **Ichibu devices**: Motor control, hatch operation, photo eye sensors, PIN management, dispensing timeouts
- **Libra scales**: Phidget load cell integration, calibration coefficients, noise filtering, sampling configuration

## Usage

### Basic Device Creation

```rust
use menu::{Device, Model};

let device = Device::new(Model::LibraV0, "L001");
```

### Configuration Management

```rust
use menu::{Libra, Config};

// Create a new scale configuration
let mut libra = Libra::default();
libra.config.ingredient = "Chicken Wings".to_string();
libra.config.location = "Kitchen Station 1".to_string();

// Save to configuration file
Libra::new_config_file(vec![libra], &path)?;
```

### Backend Communication

```rust
use menu::backend::{ConfigBackend, write::*};

let backend = ConfigBackend::new(
    "https://api.caldo.com/config".to_string(),
    "your-auth-token".to_string()
);

// Register a new device
let device = backend.make_new_device(Model::LibraV0, config)?;

// Retrieve device configuration
let config = backend.get_config(device)?;
```

## Features

The library uses feature flags to enable optional functionality:

- `write`: Enables configuration generation and backend write operations
- `address`: Enables device address management

Enable features in your `Cargo.toml`:

```toml
menu = { version = "0.1.0", features = ["write", "address"] }
```

## Configuration

### Ichibu Device Configuration

Ichibu devices are configured via `config.toml` with settings for:

- **Device**: Model and serial number
- **Conveyor Motor**: Speed, acceleration, scaling parameters
- **Hatch**: Input pins, motor configuration for opening/closing
- **Photo Eye**: Input pin, sampling parameters for object detection
- **Security**: PIN codes for different access levels (manager, operator, sudo)
- **Dispensing**: Timeout settings
- **Setpoints**: Empty weight thresholds and filling parameters

### Libra Scale Configuration

Libra scales can be configured via `scale.toml` with:

- **Phidget ID**: Hardware identifier for the load cell interface
- **Coefficients**: Optional calibration coefficients for weight calculations

## Architecture

- `device.rs`: Core device types and serialization
- `backend.rs`: REST API client for cloud communication
- `libra.rs`: Scale-specific configuration and file management
- `ichibu.rs`: Dispenser-specific configuration structures
- `error.rs`: Comprehensive error handling
- `read.rs`: Configuration file reading utilities
- `generate.rs`: Configuration file generation utilities

## Testing

The library includes comprehensive unit tests with mocked HTTP responses for backend integration testing.

```bash
cargo test
cargo test --features write,address
```

## Internal Use

This library is designed specifically for Caldo's internal kitchen automation systems and is not intended for external distribution.