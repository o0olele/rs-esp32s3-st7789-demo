﻿# rs-esp32s3-st7789-demo

A simple demo drawing text on esp32s3 and st7789 lcd with rust.

## requirements

- follow this [demo](https://github.com/ivmarkov/rust-esp32-std-demo/tree/main) and set up the build environment.

## build

~~~shell
cargo build --target xtensa-esp32s3-espidf
~~~

## flash

~~~shell
espflash flash target/xtensa-esp32s3-espidf/debug/rs-esp32s3-st7789-demo
~~~

## monitor

~~~shell
espflash monitor
~~~

![hello](./img/helloworld.png)
