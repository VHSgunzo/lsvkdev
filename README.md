# lsvkdev
Show Vulkan deviceName and driverName on Rust

## To get started:
* **Download the latest revision**
```
git clone https://github.com/VHSgunzo/lsvkdev.git && cd lsvkdev
```

* **Compile a binary**
```
rustup default nightly
rustup component add rust-src --toolchain nightly
cargo install cross
cross build --release
```
* Or take an already precompiled binary file from the [releases](https://github.com/VHSgunzo/lsvkdev/releases)

* **Usage**
```
./lsvkdev <arg>
no arg  Show all Vulkan devices names
-c      Show current Vulkan device name
-d      Show current Vulkan device driver name
-ad     Show all Vulkan devices driver names
-h      Show this usage info
```
