+++
title = "Wireless communications using embedded Rust, radio transcievers, and cheap boards from AliExpress"
author = "doomy"
template = "page.html"
date = 2020-08-10T00:34:55.312Z
description = "Draft"
+++
The Internet of Things sucks. Products often feature bad security and awful proprietary apps. Worse, some fancy-internet-stuffed gadgets don't work at all without an external internet connection. Being said, I still own a ton of these devices. I just replaced many of my home's cieling can lights with "smart" Wiz lighting, because when smart devices *do* work, they are pretty convenient.

I have an excess of sensors, microcontrollers, and time, so I sought out my own internet-connected solution. Sort of. I have a few Nordic NRF24L01 transciever modules. These cheap (~2 USD) boards can both send and receive wireless messages.

![The NRF24L01 Radio Transciever module with wires plugged into its header pins](/static/uploads/img_20200809_222058.jpg)

By the way, I'm still a beginner in embedded. Much of what I'm sharing is either parsed together from obsecure git repositories or gathered from first-hand experiences. That means you shouldn't consider me an expert on the subject. ...And that's a fantastic segue into the most important part of this ""blog"" post.

## Where to get help

If you're running into an issue that you just can't figure out, the people in a few communities are very helpful. I owe any successes in this project to the awesome folks in the Rust world. Here are a few places you can look -

- [Rust Community Discord - #embedded](https://discord.gg/VGxEtjH) - Great place for less specific questions for embedded development
- [Rust Embedded Matrix chat](https://matrix.to/#/!BHcierreUuwCMxVqOf:matrix.org?via=matrix.org&via=chat.berline.rs&via=fairydust.space) - Very active and great for general questions regarding anything embedded related
- [probe-rs Matrix chat](https://matrix.to/#/!vhKMWjizPZBgKeknOo:matrix.org?via=matrix.org&via=chat.berline.rs&via=oxide.computer) - Chat specifically for the `probe_rs` toolchain, including the `cargo embed/flash` commands. We'll talk more about this in a bit.
- [RTIC Matrix chat](https://matrix.to/#/!yafYEipFNsXDdwiHMT:matrix.org?via=matrix.org&via=chat.berline.rs&via=hackerspaces.be) - Chat specifically for discussion RTIC (formerly RTFM. No, not that RTFM). We'll talk more about this later too.

## What we'll need and where to buy it

For this project, I'm using two boards, but you can choose either or use a completely different one. I'll be providing the code I made for both. It doesn't really matter what you choose to go with, but the "Blue Pill" is slightly cheaper.

1. STM32F103C8T6 (AKA "Blue Pill", AKA "CS32F103C8T6", AKA "mystery surprise board!")
2. STM32F411CEU6 (AKA "Black Pill")


I'm not an authority on this whole "very small computers" subject, but I can offer my own, very subjective and likely unreproducable - experiences with buying these components.

### Amazon

I've orded 2 "Blue Pill" boards off Amazon, and they appeared to have real ST chips. I haven't yet ordered a F411. (For the record it seems like the HAL for each series seems to be somewhat interchangable, so code for a STM32F411 and a STM32F401 should be similar, to my understanding).

1. [STM32F103C8T6 on Amazon](https://www.amazon.com/s?k=stm32f103c8t6&i=electronics)
2. [STM32F4... on Amazon](https://www.amazon.com/s?k=STM32F4&i=electronics&ref=nb_sb_noss)


### AliExpress

Internet dollar-store has both boards, and apparently quite a few clones. I got both clone F103 and real F411 boards from AliExpress. 

1. [Clone CS32F103 Boards on AliExpress](https://www.aliexpress.com/item/32478120209.html?spm=a2g0s.9042311.0.0.27424c4d9N1nvk) - At less than 2 USD, these are pretty cheap. I haven't had issues with mine doing simple work, but your results may vary. The real ones aren't that much more expensive.
2. [Recommended Purchase Link for F4 series](https://www.aliexpress.com/item/4000346695587.html?spm=a2g0s.9042311.0.0.69e74c4dGGJqfW) - The F401 and F411 ("Black Pill") maker, [WeAct](https://github.com/WeActTC/MiniF4-STM32F4x1) recommends this specific store on AliExpress.

### Other places

If you know a good place to get either of these boards, or others, that you think might be helpful, please let me know.