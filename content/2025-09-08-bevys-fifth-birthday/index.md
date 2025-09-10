+++

title = "Bevy's 5th Birthday"
author = "doomy"
description = "It's already been another year?"

[taxonomies]
tags = ["rust", "gamedev", "bevy"]

+++

It's been a whole year since last year. Wow! Bevy is now 5 years old (and a fellow Leo). I'm not a professional developer of any capacity, but I've been a hobbyist for over a decade now (frightening to ponder!). At this point, I can confidently say the vast majority of my time programming has been with Bevy. So, what has been going on in this past year, from a humble library user?

## Bevy's 5th Game Jam: Toppled

This year's jam was the most successful Bevy jam to date, if the metric entries (98!). My submission [Toppled](https://itch.io/jam/bevy-jam-6/rate/3627478) placed 2nd overall and 1st in game design, which I'm rather pleased with[^flawed]! While I [once again have wasted copious amounts of time on random nonsense](https://doomy.org/bevy-jam-4#the-wretched-skybox-journey) and crammed a good portion of development time into a 36-hour-straight bender, I *did* manage to get a release up for web.

<iframe frameborder="0" src="https://itch.io/embed/3627478?bg_color=343138&amp;fg_color=ffffff&amp;link_color=945bfa&amp;border_color=5c5960" width="100%" height="167"><a href="https://1-doomy.itch.io/toppled">Toppled by doomy</a></iframe>

In comparison, my previous entry "Bevy Blast Ultra" - only available as a downloadable executable - received just 7 ratings, contrast to Toppled's 33.

### Bevy's UI is extremely promising

Bevy's UI system is limited, though that is improving with the addition of core widgets. Despite the complexities of figuring out reactivity, having no [`dyn Bundle`](https://github.com/bevyengine/bevy/pull/19761) to work with, and RA's tendency to just give up after a certain number of nested `children![]`, using an ECS-driven UI system is actually quite pleasant. Bevy's UI can be expressive, albeit not very ergonomic (yet). In the future, BSN[^bsn] will enable rapid iteration as UI descriptions can be described as assets and reloaded without recompilation, which solves most of my qualms. Recompiling for every UI change can get exhausting. I can only imagine how my poor computer feels about it.

{{ loop(src="ui.mp4") }}

Bevy's UI is now my go-to for game interface, replacing the venerable `egui` in most cases. `egui` is still far better suited for UI-heavy games (or really anything that needs text input), but its design is seemingly always in conflict with ECS concepts.

### Save files and scenes are difficult

I find it is currently easier to understand and work with my own scene representation as opposed to Bevy's, especially when I need to know *what* is in a scene file before spawning. In Toppled, I built my own scene asset representation with an enum of possible game objects. These game objects, when inserted, build the necessary bundle via component hooks. This makes level editing fairly straightforward, which is important for a "bridge constructor" type game like Toppled. I'm hopeful that BSN can improve the ergonomics of world serialization and open up new possibilities in this area.

## Scripting with an ECS

I've been developing an iOS game with Bevy[^app] for the past few months on and off - a combination of Peggle and Balatro. It's not at all original, but it's rather fun. Uninterestingly, this marks my 3rd game centered around physics balls, after Toppled and Bevy Blast Ultra.

This game, which I'll call Plinketto[^rlm], is the same setup as Peggle. Players fire a ball at pegs, with the goal of reaching an ever-increasing score. Players can also purchase addons to attach to balls and make them do weird things like reverse gravity, split apart, or connect pegs together with a line of more pegs. This sort of behavior would be laborious at best to work out in Rust, so I'm using the excellent `bevy_mod_scripting`[^scripting] to integrate `lua` and define behavior for any game item. For example, the following turns any hit peg into a ball of its own that can also score. This is relatively complex behavior, completely defined within lua and enabled by the emergent behavior natural to an ECS.


```lua
function on_score(ball_entity, peg_entity, points)
    -- Only bonus pegs will turn into balls
    local has_objective = world.has_component(peg_entity, types.Bonus);
    if has_objective then
        -- Adds physics to the peg so it is affected by gravity
        world.insert_component(peg_entity, types.RigidBody, construct(types.RigidBody, { variant = "Dynamic" }));
        -- Tells the ECS this peg can score other pegs
        world.insert_component(peg_entity, types.Projectile, construct(types.Projectile, {}));
        local velocity = world.get_component(ball_entity, types.LinearVelocity)._1;
        world.insert_component(peg_entity, types.LinearVelocity, construct(types.LinearVelocity, { _1 = -velocity }))
    end
end

```

*(Please forgive my lua, as the last time I wrote it was for Garrysmod SWEPs in 1998.)*

{{ loop(src="peglatro.mp4") }}

I think that's pretty powerful! I wrote code for this game *without* any thought of implementing an item like this one, but I was still able to easily extend with minimal changes by scripting and adding a few components.


## The future of Bevy's audio

Besides game development, I [first was interested](https://doomy.org/creating-an-audio-plugin-with-rust-vst/) in Rust for its potential in realtime audio processing. I've been hopeful that the RustAudio and Bevy servers would collide more, and I'm getting my wish with [Firewheel](https://github.com/BillyDM/firewheel) and its Bevy integration, [`bevy_seedling`](https://github.com/corvusprudens/bevy_seedling). This system is highly flexible and extensible, with access to an audio graph complete with effects and adjustable parameters. Additionally, buffers of audio can be directly manipulated. If you've developed an audio plugin before, you should feel at home creating a Firewheel node [^bevy_audio].

This opens up some intriging possibilities, including my nascient interest in physical modeling. Generative, physics-driven audio offers far greater control when compared to simple sample playback; sounds can be adjusted based on their physical model's properties.

I've provided an example sandbox which implements a simple [*Karplus-Strong*](https://en.wikipedia.org/wiki/Karplus%E2%80%93Strong_string_synthesis) synthesizer - a *very* simple physical model of an idealized stringed instrument. Use the left mouse to create rectangles. A ball[^balls] will pop out regularly, and create a tone for each rectangle it hits, taking into consideration the rectangle's size as a physical parameter to calculate the resulting pitch. Use the right mouse to remove rectangles. Press "R" to reset the simulation[^sim].

*Nothing* here is sampled - it's all generated in real time.

<iframe frameborder="0" src="https://itch.io/embed-upload/14905959?color=333333" allowfullscreen="" width="100%" height="480"><a href="https://1-doomy.itch.io/karplus">Play Karplus on itch.io</a></iframe>

I think it's pretty neat to get such a convincing and *reactive* sound from something so relatively simple in implementation, don't you agree? This kind of synthesis has many applications for games and interactive art. Imagine for a moment that you me, creating another game involving a rolling ball. Instead of downloading a few dozen `.wav` files, and hooking it up to a sampler, wouldn't it be neat to define material properties of your physics objects, and procedurally generate rolling noises based on just those details?

Some may consider this unnecessary CPU usage, or a complex solution to a simple problem that would do just fine with traditional sampling. Or, perhaps it's simply art that makes you slightly warmer.

## Contributing still feels daunting

I haven't contributed[^contributing] to the engine itself in any capacity despite using Bevy for a while. While I am comfortable with the engine itself, even issues marked suitable for new contributors seem like they require a unattainable level of foundational knowledge. In reality, I know it's far more beneficial for maintainers in the long run to help others effectively contribute, but the potential to burden someone with a misguided PR no matter how kind and supportive they are is somewhat of a core and primal fear of mine. I don't have a solution to this one, it's squarely a "me" problem. But I'm sure this made someone out there feel seen[^doit].

---

[^flawed]: Itch's ratings tend to punish games that have more votes. I can't lie - I'm happy to get Good Numbers, even though I more or less got lucky with the number of votes I received - but still, there are tons of [incredible games deserving a playthrough](https://itch.io/jam/bevy-jam-6/entries).

[^bsn]: Interesting fact: "BSN" is pronounced "Bee Ess N".

[^app]: I am the world's first person to start making an app and then write a blog about it.

[^rlm]: Until either CBS or Mike Stoklasa send me a C&D

[^scripting]: I'm incredibly grateful to all the helpful people in the `bevy_mod_scripting` channel for their support getting set up. This is the first time I've built scripting support into *anything* and I owe all of my success to the knowledge shared in that community.

[^bevy_audio]: Like with scripting, I'm incredibly grateful to the folks in Bevy's discord, chiefly BillyDM and Corvus Prudens for their help and patience with Firewheel and seedling.

[^sim]: I also added modal synthesis, which you can try out by tapping the "m" and creating a rectangle.

[^balls]: We're on number 4 now.

[^contributing]: I've given reviews and feedback, and while those *are* contributions and (hopefully) helpful, they are not something I personally view as significant.

[^doit]: Go ahead and contribute, it's fine. I promise I'll do it too, eventually. Probably.


> Note on SubStates:
>
> In [an older post here](/extending-states-in-bevy) I demonstrate how to override a state enum's `PartialEq` and `Hash` implementation to only consider the discriminant, assuming any contained data is equal. While this can still be useful in certain situations, I've moved to [`SubStates`](https://docs.rs/bevy/latest/bevy/state/state/trait.SubStates.html), which fit the needs of the vast, vast majority of projects, and are much easier to use.
>
> In general, the problem I was trying to solve is that it can be difficult to know when certain world information like resources are available. Instead of assuming a `Score` resource exists when loading a game save, overridden states can provide all the necessary state information in the transition itself, which can make reasoning about lifecycles far simpler. However, it comes at a cost of high maintience. You can start to devise some very clever (and complex) systems just to handle this sort of behavior, but I believe all of this is orthogonal to Bevy's usual design patterns.  `SubStates`, combined with scoped entities and systems operating on event transitions, are the idiomatic answer.
