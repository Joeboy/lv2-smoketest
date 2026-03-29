# lv2-smoketest

`lv2-smoketest` is a Rust command-line tool for inspecting and smoke-testing LV2
plugins. I wanted something like this for testing cross-platform plugin builds
on CI, and as far as I can see it doesn't otherwise exist.

## Usage

List all installed plugins:

```sh
lv2-smoketest list-installed-plugins
```

Show all metadata for an installed plugin:

```sh
lv2-smoketest show-plugin-metadata <plugin-uri>
```

Instantiate and run a plugin once to verify that `run()` completes successfully:

```sh
lv2-smoketest test-plugin-run <plugin-uri>
```

Optional flags:

```sh
lv2-smoketest test-plugin-run <plugin-uri> --sample-rate 48000 --frames 1024
```

Notes:

- Prints a success message on stdout when `run()` completes, and exits with
  code 0.
- Connects zero-filled audio/CV buffers and default-valued control inputs before
  calling `run()`.
- Will fail if the plugin has required features that we don't support.
- All we're testing is that the plugin's run method works / doesn't crash with
  default inputs.

## Build

`lilv` links against system LV2 libraries. Install these before building:

- Debian/Ubuntu: `sudo apt install pkg-config liblilv-dev`
- Fedora: `sudo dnf install pkgconf-pkg-config lilv-devel`
- Arch: `sudo pacman -S pkgconf lilv`

Then build as normal:

```sh
cargo build
```

## AI disclosure

Developed with the assistance of LLMs.
