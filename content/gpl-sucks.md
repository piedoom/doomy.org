+++
title = "The GPL sucks."
date = 2016-10-15
category = "opinion"
tags = ["programming", "legal"]
+++

In all of the repositories I've published on GitHub, not a single one is licensed under the GPL - provided that I was the original creator.  That's because the GPL license sucks, and you shouldn't use it if you really care about truly *free* software.

I'm not going to lie, I think proprietary software can be great.  I don't think we all need to live in a post-capitalist programming society where we buy groceries through bartering gists.  I don't think software that isn't freely licensed is inherently the work of evil corporate America.  FOSS is awesome, but forcing it down others' throats isn't productive or in the spirit of free software.

Have you ever seen this quote before?
> Free as in freedom, not as in beer

It's a phrase often associated with FOSS software; particularly software that uses the GPL, which - as it turns out - is especially ironic.

For those who haven't yet run into the GPL - imagine this - let's say you're using a software library licensed under the GPL for a project.  This is what you'd need to keep in mind.

#### 1. If you write some improvements to the library, you *have* to publish them

This by itself isn't too bad!  It's actually most of what the LGPL license states (and that's why the LGPL isn't too bad).  Although some might not appreciate the fact that they *have* to publish any changes they make to your code, for the most part this isn't such a big deal.  But wait!

#### 2. If you use the library in your code, you also need to license it under the GPL.

This means that all code linked with GPL source code must be distributed under a GPL compatible license, so you can't make *any* proprietary projects.  This may not be a huge problem, especially for smaller non-commercial projects, but this stipulation shows what is really wrong with the GPL.  It's not about freedom; it's about forcing anyone who uses it into the same restrictive license, whether the author likes it or not.

A fair argument might be that companies have less incentive to contribute to FOSS communities provided they aren't legally obliged.  While there doesn't seem to be statistical evidence to back up this claim, a quick look at top tech companies on GitHub seems to prove that big companies *do* give back.  [Microsoft](https://github.com/Microsoft) is one of the largest contributors to FOSS on GitHub.  Netflix is a huge contributor to FreeBSD despite the OS being licensed under the far more permissive BSD license.  Look further on GitHub, and you'll see an incredible amount of companies giving back to free software communites, whether or not they are bound to a restrictive license.

#### So what if I chose the GPL?

Well, for starters, the GPL explicitly says that you cannot revoke the GPL, even if you own the project.  I recommend reading this [groklaw article](http://www.groklaw.net/article.php?story=2006062204552163), as IANAL, but they seem to be one!

# Good, actually free licenses!

If you believe in truly free software, I recommend the MIT or BSD license.  Put very basically, all they state is "You can use this code however you like, but you can't pretend you wrote it."  Nice.

