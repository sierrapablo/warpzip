## WarpZip

---

**WarpZip** is a high-speed compression and decompression tool for various formats, designed to be fast and efficient. It supports multiple compression formats and provides a simple and intuitive command-line interface.

---

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Flags](#flags)
5. [Contributing](#contributing)
6. [To Do's / Future Features](#to-dos--future-features)
7. [License](#license)

---

## Features

- Compress and decompress files in various formats (e.g., `.zip`, `.tar.gz`, `.xz`, `.zst`, and more).
- Automatic format detection during decompression.
- Control over compression levels and output directories.
- Support for password-protected archives (ZIP only).
- Ability to list archive contents before extraction.
- Supports simulating actions with the `--check` flag.

---

## Installation

To install **WarpZip**, clone this repository and build the project using **Cargo** (Rust's package manager and build system).

1. Clone the repository:
   
   ```bash
   git clone https://github.com/your-username/warpzip.git
   cd warpzip
   ```

2. Build the project:
   
   ```bash
   cargo build --release
   ```
   
   After building, the executable will be available in the `target/release/` directory.

---

## Usage

### `warp` - Compress files or directories

Compress a folder:

```bash
warp folder -xz -o output-file
```

Compress with a specific compression level:

```bash
warp folder -xz -lvl 9 -o output-file
```

### `unwarp` - Decompress files

Decompress a file:

```bash
unwarp file.tar.xz -o output-folder
```

List the contents of an archive:

```bash
unwarp file.tar.xz -ls
```

### `wrp` - Short version of `warp`

```bash
wrp folder -xz
```

### `unwrp` - Short version of `unwarp`

```bash
unwrp file.tar.xz
```

---

## Flags

| Flag         | Alias  | Description                                               | `warp` | `unwarp` |
| ------------ | ------ | --------------------------------------------------------- | ------ | -------- |
| `--abc`      | `-abc` | Select compression format (zip, tar, gz, xz, zst)         | ✔️     | ✔️       |
| `--output`   | `-o`   | Output file or folder name                                | ✔️     | ✔️       |
| `--help`     | `-h`   | Show help                                                 | ✔️     | ✔️       |
| `--erase`    | `-e`   | Eliminate the compressed file after successful extraction | ✔️     | ❌        |
| `--verbose`  | `-v`   | Detailed output                                           | ✔️     | ✔️       |
| `--force`    | `-f`   | Force overwrite existing files                            | ✔️     | ❌        |
| `--lvl`      | `-lvl` | Set compression level (1-9)                               | ✔️     | ❌        |
| `--check`    | `-c`   | Simulate the operation without actually performing it     | ✔️     | ✔️       |
| `--ls`       | `-ls`  | List contents of the archive without extracting it        | ❌      | ✔️       |
| `--password` | `-p`   | Set password for encryption (ZIP only)                    | ✔️     | ✔️       |
| `--quiet`    | `-q`   | Suppress output                                           | ✔️     | ✔️       |

---

#### Contributing

For guidelines on contributing to **WarpZip**, please check the [CONTRIBUTING.md](CONTRIBUTING.md) file.

---

## To Do's / Future Features

| Feature                    | Description                                                                                 | Priority | Status      |
| -------------------------- | ------------------------------------------------------------------------------------------- | -------- | ----------- |
| **Multi-format support**   | Allow compression and decompression of more formats (e.g., `.tar.gz`, `.zip`, `.rar`, etc.) | High     | Planned     |
| **Parallel decompression** | Implement parallel decompression to speed up the process for large files                    | Medium   | Planned     |
| **Encryption support**     | Add support for encrypted archives (e.g., password protection for `.zip` files)             | Low      | Planned     |
| **Progress bar**           | Implement a progress bar for compression and decompression operations                       | Medium   | Planned     |
| **Better error handling**  | Improve error messages and handling for various edge cases                                  | High     | In progress |
| **Automatic updates**      | Implement an automatic update feature for the tool itself                                   | Low      | Planned     |
| **CLI enhancements**       | Add additional flags for fine-grained control of compression levels, etc.                   | Medium   | Planned     |
| **Cross-platform support** | Ensure compatibility with Windows and macOS platforms                                       | High     | Planned     |

For new feature suggestions or ideas that are not listed here, feel free to reach out to me directly at **[pablosierralorente@gmail.com](mailto:pablosierralorente@gmail.com)**. I welcome discussions and contributions to improve the project.

---

## License

**WarpZip** is licensed under the GNU General Public License (GPL) License. See the [LICENSE](LICENSE) file for more information.

---
