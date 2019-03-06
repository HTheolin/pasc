# PASC software

## How To: Debug/Program

```
$ cargo build
$ openocd -f openocd.cfg
```

```
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/Software -x openocd.gdb
```