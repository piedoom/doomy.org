+++
title = "I don't want to hang out with the AI guys."
author = "doomy"
description = "The death of creative spaces"

[taxonomies]
tags = ["rust", "gamedev", "bevy"]
+++

> I love Bevy. This is my 3rd [birthday](/bevys-fourth-birthday) [post](/bevys-fifth-birthday) in spirit only, as I have no intention of submitting as usual. 

Over the past year, I've done something mythical; make friends as an adult. It's hard to meet others, outside of the familiar epicenters of school, or university if you're lucky enough. It was a shock when I graduated from the incredibly technical environment, to which I'd grown accustomed in college, and folded back into normal society where dunking on Python is a less reliable opener. Bevy's community has been a wonderful replacement, full to the brim of characters who share passions similar to my own.

It has been wonderful for me. Has been. And maybe it still will be. But it's no longer a place I want to promote or share endeavours I deem artistic, because of (who would have guessed) the tech we're currently referring to as "AI". Being inspired and making friends isn't easy when your community is chock full of antisocial technology.

Earlier this summer, Bevy adopted a new (more permissive) AI policy. Instead of a full and complete ban, AI contributions to the engine are now allowed, given that the contributor marks it and understands the changes fully. This change was made because frankly, a full ban is impossible to enforce. AI contributions still make it into the engine, but the "don't ask don't tell" policy means investigating any suspect PRs is nearly impossible.

I like the new rules! I actively voiced concerns and saw them addressed in real-time. If I had a vote and could do it again, I'd still want the same outcome. This isn't because AI is inevitable, but because by the time new guidelines were *required*, AI pushers already had Bevy's community at gunpoint. They were already causing issues, flaming other members with concerns, and overusing moderation channels to carve out their space.

Bevy missed its opportunity to state its values and actively discourage AI use that could have helped avoid the "inevitability" of this problem. It has been done effectively. Creating spaces free of copyrighted, stolen, or at the very least creatively-bankrupt material is very possible. But, this was never a goal. One of the project leads, for whom I have great respect, has repeatedly stated her pluralistic values, to the point where she indicated that a continued ban would result in her resignation, which is understandable seeing how much stress constant AI moderation issues cause. 

So, that's where we're at. The loudest, most obnoxious people who hang in the wretched alleys of `#machine-learning` got their way. Not because they can now contribute their slop (they've already been doing that without disclosure), but because it's now essentially against the rules to make AI look "uncool", which was one of the last tools used to push back [^1]. They got there not by explaining the virtues of their technology, but savagely brow-beating (or coyly goading) those opposed. Bevy's community had *very* few moderation issues minus the occasional Mr. Beast spam post - even political chat was allowed. The consistent stressor was always discussions about AI.

So we gave up, and gave in.

## Why not?

Why not generative AI? It makes you faster. It makes you more productive. It makes it so you can have all options at your fingertips. It helps you write the things you don't want to write, and read the things you don't understand. It helps you build your vision, but faster...?
I don't give a fuck, I don't care. I don't want to see AI slop [^2]. I'm not interested that someone can write a few english sentences to Claude and have it produce physically accurate ocean waves based on tidal currents. Advocates can make 1,000 different arguments concluding my position is illogical - that I place too much regard on humanity, or don't understand the technology. It's besides the point. I don't care. I have no logical argument to prove to these people, and if I did, they'd shift the goalposts elsewhere. There's no point in arguing.

I don't want AI pushers in my community for no other reason than *I just don't like them.* I'm passionate about the aggressive protection of human spaces, and will continue to advocate that the Rust Audio org is steadfast in its hardline stance against AI.

---

## The damage

Let's run some unscientific numbers to illustrate why I'm checked out: let's go through the latest posts in `#showcase` and see how many are AI-generated. This is my blog, so I'm absolutely witch-hunting and checking each repo when source is available [^3]. Out of the past 30 showcase submissions, around 10 can be confirmed as AI generated. If source was unavailable, I erred on the side of "No AI".  That's a 1 in 3 chance of being exposed to content I actively hate. Those numbers might be even worse than some social media platforms. 

I don't want to see AI art. Artists don't want to see AI art. The forces that drove artists off of platforms pushing AI like DeviantArt and ArtStation remain consistent all the way down to small Discord communities. Creative communities die when AI pushers are allowed to take the same stage. Protect yours.

[^1]: Insofar as to create a rule (which I am fairly certain is targeted at me specifically) prohibiting "unconstructive negative ... reactions in response to contributors or community members who disclose AI use", as I would often react with a robot emoji on AI generated showcase posts.

[^2]: "Well done" slop is still slop. I'm as interested in nice looking AI clouds and grass as I am an AI generated shitstain. I don't really care what you create, you should keep it to yourself when around others, like a noxious fart.

[^3]: I'm checking for fairness. In reality, distinguishing AI generated content is usually effortless unless the person sharing went through great lengths to rewrite their visuals, code, and docs. There are several tells with Bevy projects, the first being [^4] that the author has no idea how their code works and cannot explain it, the second being that Claude loves to abuse text UI for debug information.

[^4]: The first tell is actually that it looks like uninspired shit, but I'm being nice.
