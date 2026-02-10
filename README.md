# FUM

Rename `FirmwareUpdateMode` to `FirmwareUpdateModeRT` as a workaround for
<https://github.com/Dasharo/dasharo-issues/issues/1759>

## Preparation

```sh
rustup target add x86_64-unknown-uefi
```

## Build

### Debug

Contains additional debug prints.

```sh
cargo build
```

Output file:

```text
target/x86_64-unknown-uefi/debug/replace_fum_efivar.efi
```

### Release

```sh
cargo build --release
```

Output file:

```text
target/x86_64-unknown-uefi/release/replace_fum_efivar.efi
```
