+++
title = "Rewriting Resamplr: Goals"
date = 2018-03-09
category = "development"
+++

I started programming as a design student in college with Python and Ruby.  While great languages with purpose, I've recently found myself less and less happy with dynamically typed languages.  

Specifically, Ruby on Rails can cause headaches.  Ruby's concept of "truthiness" has led me to all sorts of confusing code.  I might just be a poor Ruby programmer, but I prefer rigid compiler checks over runtime errors.

It's no secret that I've grown to love the [Rust programming language](/tags/rust/) lately.  I started learning it because [I wanted to be in on the joke](https://reddit.com/r/programmingcirclejerk), but the more I learned, the more I liked it.  Going back to maintain my project [Resamplr](https://resamplr.com) felt miserable after Rust.  So I decided to trash it, and redo it.

# Project goals
In order to avoid the same pitfalls as before, I set up goals for the new project.

- It should separate back-end logic from front-end views
- It should be modular and easily maintained
- It should focus on user security & experience
- It should be tested thoroughly 
- It should deploy easily (no manual `git clone` on the production server)

# Starting Over: Fuck JavaScript
Rails projects (usually) mix front-end views with back-end logic through some templating system.  This often felt weird and messy.  I began by researching alternatives.  I came across a few contenders: React, Elm, and PureScript.

## React
Pros:

- Huge community
- Gentle learning curve

Cons:

- Still JavaScript
- Absolutely *huge*
- Tooling sucks

## PureScript
Pros: 

- No runtime errors
- Purely functional
- Healthy community

Cons:

- Purely functional
- Reactive frameworks slow comparatively
- Similar to Haskell in learning curve (i.e. super hard)

## Elm
Pros:

- No runtime errors
- Great introduction to functional programming
- One of the fastest reactive frameworks

Cons:

- Can be verbose and full of boilerplate
- Smaller community
- Rigid architecture 

In the end, I chose Elm because of its speed.  Although PureScript looks great (and I intend to learn more about it someday) it proved too difficult for my little imperative mind to understand.  Halogen, PureScript's go-to reactive framework, is also much slower than Elm, and even slower than React.

## End
I will continue to document my findings with Rust and Elm in the coming months.  In the meantime, you can find the WIP source code [on Github](https://github.com/resamplr/resamplr-web).