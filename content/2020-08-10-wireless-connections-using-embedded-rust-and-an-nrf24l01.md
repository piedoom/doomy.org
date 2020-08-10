+++
title = "Wireless connections using embedded Rust, an NRF24L01, and an STM32 MCU"
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

The best part about developing in Rust is the community. Others
