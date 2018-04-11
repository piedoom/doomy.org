+++
title = "Kontakt: A Case Study"

# The date of the post.
# 2 formats are allowed: YYYY-MM-DD (2012-10-02) and RFC3339 (2002-10-02T15:00:00Z)
date = 2017-10-04

# An overarching category name for that page, allowing you to group pages with it
category = "design"


# Use aliases if you are moving content but want to redirect previous URLs to the 
# current one. This takes an array of path, not URLs.
aliases = ["kontakt-a-case-study"]

# Template to use to render this page
template = "page.html"
+++

This is an analysis of *Native Instruments'* Kontakt 5 software sampler audio plugin.  This is more of an idea scratchpad instead of a fully-thought-out post.

# User facing

![](/images/kontakt-a-case-study/1.png)

UI has since been updated but Native Access is a heap of garbage that can't stop crashing, so I can't update!

- Easy to navigate file browser that for once is welcome over Window's default
- Layout is simple (for the most part)
- Kind of boring grey tones.  We don't want something exciting, but the half-grey with poorly rendered text gives a strong 2000s vibe.

![](/images/kontakt-a-case-study/2.png)

- Reference tone is useful when tuning samples
- Features are haphazardly laid out and difficult to scan.  More sectioning could be done to make this better.  I also don't like this taking up vertical space.

![](/images/kontakt-a-case-study/3.png)

- Multi-rack saves memory at the expense of ease of use (at least in a DAW).  I'd rather have multiple instances of a plugin running than one master - it feels very MIDI hackish (because it is!).

- Default controls look straight out of 2001
- Text rendering is awful
- Rack idea is just overall a bit convoluted and unnecessary 

![](/images/kontakt-a-case-study/4.png)

- Nice preview information for a given instrument
- More text rendering problems

![](/images/kontakt-a-case-study/5.png)

- Items take up too much vertical space and quickly become annoying
- Would make more sense to have a two-pane system instead of having outdated accordian-style tabs
- Very difficult to use when in possession of many libraries

# Technical Overview

- Heavily DRM dependent to a huge fault
- Libraries often disappear and are difficult to retrieve without reinstalling the whole product
- Native Access is a garbage heap that never works.  The plugin should be self contained.  While a manager to download updates can be nice, it shouldn't interfere with the core experience.
- User-made libraries have no copy protection unless a contract is formed with NI.  This is actually a good incentive for commercial industries to purchase a license, but as of now that license is cost prohibitive for most individuals.  NI is missing out on a market of self-made producers.
- The KSP language is an absolute nightmare to work with and makes me suicidal.  It's like a combination of COBOL and PHP.
- The editor is too split between GUI and programmatic elements, and they often butt heads.  Most work should be done in scripting with the exception of zone editing and (possibly) user-facing GUI editing.

# Bare Elements
- File browser & Library
- GUI Editor
- Zone/Mapping editor
- Sample/Looping editor
- Group editor (useful for stuff like round robins/velocity/etc).
- Script editor

# Useful additions
- downloads within program
- easier publishing and packing