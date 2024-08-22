+++
title = "A connected display for under $5 using embedded Rust"
author = "doomy"
description = "Building a WIFI connected display using a cheap OLED panel and ESP32 C3 from AliExpress"

[taxonomies]
tags = ["rust", "embedded"]
+++

## Materials

- 0.9in 128x32 i2c oled white display ~$1.50
  https://www.aliexpress.us/item/2251832675560929.html?spm=a2g0o.order_list.order_list_main.17.5ee21802JDd88y&gatewayAdapt=glo2usa
  - Working votage: 3.3v-5v
- esp32-c3 "super mini" (be careful when soldering not to damage the board)

## Setup

I have all the esp stuff set up. Follow the esp book here - https://esp-rs.github.io/book/

You won't need a special connector like for STM32, just a USB Cable.

Cables matter, and not every one seems to work. If you arent getting a connection, try several cables.

---

Set up a project using [cargo-generate](https://esp-rs.github.io/book/writing-your-own-application/generate-project/index.html) without std or idf or whatever

```
cargo generate esp-rs/esp-template
```

**Output and options:**

```zsh
⚠️   Favorite `esp-rs/esp-template` not found in config, using it as a git repository: https://github.com/esp-rs/esp-template.git
🤷   Project Name: tinyguy # this can be whatever you want
🔧   Destination: /myuser/tinyguy ...
🔧   project-name: tinyguy ...
🔧   Generating template ...
✔ 🤷   Which MCU to target? · esp32c3 # the c3 is RISC-V and works without the Rust fork
✔ 🤷   Configure advanced template options? · false
🔧   Moving generated files into: `/myuser/tinyguy`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /myuser/tinyguy
```

Then we'll open the project and try to run it to verify.

You should see something like this upon running the project, though it may change based on your OS. This is how mine looks on Mac:

```
❯ /dev/cu.usbmodem1101 - USB JTAG_serial debug unit
  /dev/tty.usbmodem1101 - USB JTAG_serial debug unit
  /dev/cu.wlan-debug
  /dev/tty.wlan-debug
  /dev/cu.Bluetooth-Incoming-Port
  /dev/tty.Bluetooth-Incoming-Port
```

You'll want to select the `USB JTAG_serial debug unit`. If you don't see that option, it's likely that you

1. Have an incompatible USB cable
2. Accidentally destroyed the ESP32 board (happens)

I'm not aware of any driver situation on Windows, but there do appear to be some USB drivers available for this chip, so that might be worth a shot.

If everything goes well, you'll see this upon flashing:

```
Hello world!
Loop...
Loop...
Loop...
...
```

And on and on into infinity. Now we'll make it do something more fun.

## Adding crates

Chip is specified as SSD1306. Seems to be quite [popular](https://lib.rs/crates/ssd1306). We'll add that and `embedded-graphics`.

```
cargo add ssd1306 embedded-graphics
```

---

There's actually an example for what appears to be the exact dimensions of our display in the repo, albiet for the "Blue pill". We'll add add adjust the code to suit our MCU.

Everything translates well except creating the I2C instance. There's a good example in the [ESP32 docs](https://docs.rs/esp32c6-hal/latest/esp32c6_hal/i2c/index.html), thankfully. (Yes, this is the c6 docs. As of writing, the c3 docs fail to build for some reason, but they appear to be the same.)

I'm not sure why, but I have to change up the crate a bit or else the display doesn't seem to have the necessary traits to work with `embedded-graphics`. I'm guessing there is some movement right now in the expressif-rs world and they're in the process of moving to this new crate, but it doesn't have any publishes on crates.io save a placeholder.

```toml
esp-hal = { git = "https://github.com/esp-rs/esp-hal/", features = [
    "eh1",
    "esp32c3",
] }
```

By the time you read this, double check lib.rs for `esp-hal`, there might be something published.

For whatever reason, I kept getting linking errors, so I checked around and found that I need to add a `#[no_mangle]` to my main fn. After that, it works!

I don't know how burn in works with these so I wouldn't leave this on for super long.
