<p align="center">
    <a href="https://opensource.org/licenses/Apache-2.0" alt="Apache 2.0 License">
        <img src="https://img.shields.io/badge/License-Apache_2.0-orange.svg" /></a>
    <a href="https://www.rust-lang.org/tools/install" alt="Rust 1.88.0">
        <img src="https://img.shields.io/badge/Rust-1.88.0-orange.svg" /></a>
</p>

hwtemp-rs is a linux-based library for retrieving sensor temperatures in Rust. It does so by reading the temperature files from **sysfs** interface (**_/sys/class/hwmon/hwmon*/_**).

## Supported & tested platforms

The library was developed on PopOS 22.04 and tested on Ubuntu 22.04 and Raspberry Pi OS 12 (Bookworm) 

## Prerequisites

- `cargo 1.88.0`
- `rustc 1.88.0`
- `rustup 1.28.2`

## Installation
```
cargo add --git https://github.com/thedoctor095/hwtemp-rs
```
## Example usage
### Streaming mode (interval-based)
```
use hwtemp_rs::{from_stream, HWProperties};

fn fetch() {
    // retrieve temps every 2 seconds
    for temps in from_stream(Some(2)) {
        println!("Temps are {:#?}", temps);
    }
}
```
### Static mode
```
use hwtemp_rs::{from_func, HWProperties};

fn fetch()-> HWProperties {
    // retrieve temps once
    return from_func();
}
```

### Output
**Raspberry Pi 5 output**
```
Some(
    {
        "rp1_adc": [
            Properties {
                label: "",
                current_temp: 51.985,
                max_temp: 0.0,
                critical_temp: 0.0,
                sensor_path: "/sys/class/hwmon/hwmon1/temp1_*",
            },
        ],
        "cpu_thermal": [
            Properties {
                label: "",
                current_temp: 49.05,
                max_temp: 0.0,
                critical_temp: 0.0,
                sensor_path: "/sys/class/hwmon/hwmon0/temp1_*",
            },
        ],
    },
)

```
**PopOS output**
```
Some(
    {
        "acpitz": [
            Properties {
                label: "",
                current_temp: 27.8,
                max_temp: 0.0,
                critical_temp: 0.0,
                sensor_path: "/sys/class/hwmon/hwmon0/temp1_*",
            },
        ],
        "nvme": [
            Properties {
                label: "Sensor 2",
                current_temp: 36.85,
                max_temp: 65261.85,
                critical_temp: 0.0,
                sensor_path: "/sys/class/hwmon/hwmon1/temp3_*",
            },
            Properties {
                label: "Sensor 1",
                current_temp: 28.85,
                max_temp: 65261.85,
                critical_temp: 0.0,
                sensor_path: "/sys/class/hwmon/hwmon1/temp2_*",
            },
            Properties {
                label: "Composite",
                current_temp: 28.85,
                max_temp: 67.85,
                critical_temp: 70.85,
                sensor_path: "/sys/class/hwmon/hwmon1/temp1_*",
            },
        ],
        "coretemp": [
            Properties {
                label: "Core 20",
                current_temp: 26.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp22_*",
            },
            Properties {
                label: "Core 16",
                current_temp: 29.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp18_*",
            },
            Properties {
                label: "Package id 0",
                current_temp: 27.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp1_*",
            },
            Properties {
                label: "Core 4",
                current_temp: 24.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp6_*",
            },
            Properties {
                label: "Core 0",
                current_temp: 23.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp2_*",
            },
            Properties {
                label: "Core 29",
                current_temp: 26.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp31_*",
            },
            Properties {
                label: "Core 8",
                current_temp: 25.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp10_*",
            },
            Properties {
                label: "Core 31",
                current_temp: 26.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp33_*",
            },
            Properties {
                label: "Core 30",
                current_temp: 26.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp32_*",
            },
            Properties {
                label: "Core 28",
                current_temp: 26.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp30_*",
            },
            Properties {
                label: "Core 12",
                current_temp: 22.0,
                max_temp: 80.0,
                critical_temp: 100.0,
                sensor_path: "/sys/class/hwmon/hwmon4/temp14_*",
            },
        ],
    },
)
```

## Contributing

Feel free to contribute to **hwtemp-rs** by opening issues, submitting pull requests, or suggesting improvements.

## License

This project is licensed under the Apache-2.0 - see the [LICENSE](LICENSE) file for details

## Acknowledgements
Project was inspired from Python's [psutil (process and system utilities)](https://github.com/giampaolo/psutil)
