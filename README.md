# FurniFab

![build](https://github.com/adiantek/FurniFab/actions/workflows/build.yml/badge.svg)
![build](https://github.com/adiantek/FurniFab/actions/workflows/eslint.yml/badge.svg)
![build](https://github.com/adiantek/FurniFab/actions/workflows/rust-clippy.yml/badge.svg)

FurniFab is a key tool for furniture manufacturers,
enabling significant improvements in various stages of production through the use of advanced software.
This technology guarantees higher quality and production efficiency,
which would be unattainable if relying solely on manual labor.
The software is specifically configured for each stage of production,
resulting in increased throughput and better work outcomes.
To achieve that result, it uses various algorithms from the literature and original.
This combination of algorithms produces results that are better than human-made results.

## Features

- **Optimizes Supply Chains**: Founds the best solution for the needed number of materials and given suppliers.
- **Better Material Efficiency**: Reduces the number of materials that are wasted in the process.
- **Improved Efficiency**: Enhances productivity at various stages of furniture production.
- **Seamless Integration**: Facilitates communication and collaboration between departments by integrating different production stages.

## Benefits

- **Enhanced Throughput**: Tailored configurations for each production stage lead to improved throughput.
- **Better Results**: Achieves superior work results by optimizing each step of the production process.
- **Increased Collaboration**: Integrates various production phases, improving interdepartmental communication and cooperation.
- **Benchmarking Performance**: Sets a new standard for quality and efficiency in the furniture manufacturing industry.

## Download Installers

Installer files for all supported platforms can be downloaded at [https://adiantek.github.io/FurniFab/](https://adiantek.github.io/FurniFab/).

## User manual

The user manual is available at [USER MANUAL](.github/USER_MANUAL.md).

## Building

### Prerequisites

Ensure you have the following installed:

- Python >= 3.10
- Rust >= 1.77
- Node >= 20.12
- Latest Tauri CLI

Tauri CLI can be installed with:
```bash
cargo install tauri-cli
```

To build and sign packages, you need to generate a key pair:
```bash
cargo tauri signer generate
```
The above line will generate instructions on how to provide a private key during the build phase. The public key must be replaced in `src-tauri/tauri.conf.json`.

### Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/furnifab.git
    ```
2. Navigate to the project directory:
    ```bash
    cd furnifab
    ```
3. Install the required dependencies:
    ```bash
    npm install
    ```
4. Build project dev version:
   ```bash
   cargo tauri dev
   ```
5. Build installers, update packages:
   ```bash
   cargo tauri build
   ```

This repository contains a bundled Python interpreter embedded in the application when building installers.
The repository also contains compiled static libraries for all supported platforms of the C part of the project.

### More

For more details on building the project and architecture details see the [BUILDING](.github/BUILDING.md) file.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

## Contact

For more information or any inquiries, please contact us at [contact.vernite@gmail.com](mailto:contact.vernite@gmail.com).
