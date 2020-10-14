+++
title = "Using the M3D Micro in 2020"
author = "doomy"
template = "page.html"
date = 2020-10-14T19:57:08.883Z
description = "Updating the M3D Micro printer to work in 2020"
+++
Nearly 6 years ago, a 3D printing startup called M3D successfully kickstarted the [Micro](https://www.kickstarter.com/projects/m3d/the-micro-the-first-truly-consumer-3d-printer). At the time, this was marketed as one of the "first" small consumer-grade 3D printers. 

Time has passed, and the original ~300 USD price tag of the Micro seems absurd given the varied selection of cheap consumer printers currently for sale. It's painfully slow, prone to jams, has an extremely small print volume, and awful outdated proprietary software. Even so... I paid for it, and I'm going to get my money's worth.

## Making the Micro Better

Thankfully, a handful of open-source firmware updates and applications make the printing experience tolerable.

The first step is to download M33 Manager. M33 Manager will allow you to install [iMe](https://github.com/donovan6000/iMe), an open-source firmware for the M3D Micro that allows for greater print speed and control via RepRap G-Code.

> [iMe Installation links](https://github.com/donovan6000/iMe#installation)

After installation, ensure the printer driver is installed. If it isn't, there is a handy button within M33 Manager to do that for you. Hit the "Connect" button and wait for your printer to connect. If you have issues connecting, try power-cycling your Micro and trying again, and ensure no other application is taking over the device.

Once connected, you can begin the firmware update. Click the "Install iMe Firmware" button, and wait for completion. That's it! Easy.

### Some caveats

iMe will put your printer in bootloader mode whenever it is powered on. To begin printing, you will need to open M33 manager every time the printer is powered on, connect, and click the "Switch to firmware mode" button. Then, disconnect.

## Ditching the M3D Beta

### Slicing

If you're still using the M3D Micro in 2020, you'll know the last update the print software got was a beta 2016. What a great gift for us early backers! Well, we can finally junk that pile of trash software and install [Cura](https://ultimaker.com/software/ultimaker-cura). (Interestingly enough, I believe the original M3D software uses the Cura slicer anyways.) We will be using Cura to slice our STL models into usable G-Code.

After installing Cura, we'll need to adjust a few settings. 


### Sending G-Code

Now, if you happened to read the M33/iMe documentation, you'll know it says Cura should support our new firmware out of the box. However, the last version of iMe was released quite some time ago, and 3D printing has largely moved away from USB printers. I haven't had success connecting via Cura in version 4.7, even after hours of messing about with settings. So, we'll need a separate application for sending G-Code.

After testing a few solutions, I found [PrintRun](https://www.pronterface.com/). It's not a pretty application, and was last updated in 2017, but it works! I work on a Windows PC, so I installed their Windows binary. 