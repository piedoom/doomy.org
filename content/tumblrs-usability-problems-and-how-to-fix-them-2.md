+++
title = "Tumblr's usability problems - and how to fix them"
date = "2016-08-04"
category = "design"
tags = ["tumblr"]
+++

Tumblr is a fairly popular blogging website that I've used from somewhere between 4 to 5 years.  Although the network has gone under considerable change for the better, there are still an overwhelming number of usability issues.  Let's take a look!

### Post Types

![](/images/tumblrs-usability-problems-and-how-to-fix-them-2/Capture-2.PNG)

The most glaring issue of Tumblr is also the one that's been around the longest.  Tumblr uses a banner of several different post types, all with slightly different rules and quirks.  Text and Photo posts are used most often, while Chat and Quote posts are rarely used.  There's really no point to make the user decide what loose format to stick to before posting.  

Right now, I'm writing on the very nice Ghost platform.  I create a new post by clicking the "New post" button.  I'm not prompted with a massive wall of permissions - "What do you intend to post?  Are you going to post a photo?  Will there be audio?" - and I'm free to insert whatever content I desire.

Tumblr enforces arbitrary limits on certain post types that don't make very much sense.  You can only post one video at a time when selecting the "Video" option, but you can insert as many videos as you like when selecting the "Text" option.  Audio posts are limited to one file, and "Chat" posts don't work well with user comments.  Tumblr's weird post architecture seems like an artifact of poor programming and design, rather than a deliberate choice.  

How can they fix this?  Simple.  Only provide one post type.  Allow users to insert text, videos, and audio to their discretion.  I mean, the "Text" post basically does that now - so why the redundancy?

Okay - cosmetically, it's an easy change.  Under the hood, things would be much more complex.  But I think it's well worth it. 

### Hijacking Tumblr Themes

Tumblr allows users to embed custom, invisible HTML and Javascript in any post.  These custom tags won't show or execute on the user dashboard, but will execute when on a custom user page.  I've reported this to the Tumblr staff some time ago, and they have made it clear they view this as a "feature", not a usability issue.  

You might notice I hesitantly say " usability issue".  That's because when viewing a user blog, there's no valuable session cookies like on the dashboard.  However, any blog that reblogs a malicious post can be manipulated in almost any way possible.  This can be exploited so all links change their target to a malicious page, or used to style a page for phishing purposes.

This is an easy fix - don't allow script tags in posts!

### Editing Posts on Mobile 

For as long as I can remember, Tumblr has had some of the worst mobile experiences to date.   One particularly heinous usability crime is revealed when editing a post.

![](/images/tumblrs-usability-problems-and-how-to-fix-them-2/upload-949594762.jpg)

Can you guess what it is?  If you said "allowing doomy to post political opinions" you're right, but take a closer look.   That's HTML.  Mobile users have to edit HTML when editing their own posts.  This issue has been around for ages, and it's almost comical that something so simple to fix (yet so degrading to usability) is still around. 

### More mobile stuff!

As my 1-star rating of the Tumblr app might tell you, I'm not particularly fond of the Android application.  It's a mess of half-implemented and fully-missing features bundled into something that crashes every other minute.  

There's more than a bit missing from the Android application.  For example, media can't be added in text posts (unless you're into writing your own HTML on a mobile phone).  You can add your own gifs, but no images or videos.

###  Infinite scrolling and the Death of RAM

Tumblr by default implements "infinite scrolling", similar to the effects of Facebook and Twitter.  While convenient, Tumblr doesn't unload any content after scrolling, and as a result, eats up an ungodly amount of RAM.  This makes the site very difficult to use on mobile or on consoles like the Xbox one.  Tumblr does have a little setting hidden away that sacrifices endless scrolling for pagination, but endless scrolling is so *nice*, especially for the sort mindless entertainment Tumblr provides.

XKit - a popular browser extension for Tumblr - has a mode called "Hermes".  When enabled, it'll unload images when scrolled outside of the viewport.   It greatly boosts the performance of Tumblr, but it's kind of messy, and doesn't always work as intended. 

### Conclusions

I want to be clear here - I don't hate Tumblr.  I really enjoy the service, but there are definitely some problems that should be resolved.

Here's a good rule of thumb - if a site has an actively developed browser extension to improve usability, it probably can use some work. 
