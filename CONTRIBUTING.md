# Speed up compilation

Default compilation times can be relatively long. Rust is not the fastest
language to compile and `pgrx` projects specifically can take quite a long
time to compile by default. Below are two ways that can help speed up
repeated builds significantly.

## Install sccache

```bash
cargo install sccache --locked
cat >> ~/.cargo/config.toml <<EOF
[build]
rustc-wrapper = "/home/jelte/.cargo/bin/sccache"
EOF
```

## Use a faster linker

Install either `lld` or `mold` and configure it as the default linker for
rust by adding the following to `~/.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=/path/to/lld-or-mold"]
```
