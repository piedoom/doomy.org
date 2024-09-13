+++
title = "birthday_system.system()"
author = "doomy"
description = "Half a decade of Rust game engines, Bevy, project organization, and the glorious future of Rust game development"

[taxonomies]
tags = ["rust", "gamedev", "bevy"]

[extra.feather.opengraph]
image = "opengraph.jpg"

+++

![A 3D scene that reads "4 years of bevying"](opengraph.jpg)

### Table of contents

* [**4 years of bevy**](#4-years-of-bevy)
  * [Best bevy improvements](#my-favorite-bevy-improvements-since-v0-1)
  * [Organizing a bevy project for honor and success](#organizing-a-bevy-project-for-honor-and-success)
  * [Lessons from the past](#projects)
* [**Looking forward**](#iii-looking-forward)
  * [What I'm still waiting for](#what-i-m-still-waiting-for)
  * [bsn!](#bsn-bsn)
* [**Closing**](#closing-thoughts)

## 4 years of bevy

Bevy released [4 years](https://bevyengine.org/news/bevys-fourth-birthday/#bevy-ecs-maturity) ago. Feel old yet? Well, I do, because my dive into making games with Rust started even earlier than that.

5 years ago, Bevy didn't exist. The popular choices for Rust game engines were "[Piston](https://github.com/PistonDevelopers/piston)", a modular game engine which pioneered a ton of useful crates[^ui], and "[Amethyst](https://github.com/amethyst/amethyst)", a data-oriented, ECS-driven game engine, both of which are no longer in active development[^piston].

![*Amethyst, a* precursor *engine to Bevy, was archived on April 18, 2022, and is now read-only. Please ignore the fact that apparently nobody wants to talk to me on GitHub (0 notifications).*](amethyst.png)

I have a particularly strong attachment to Amethyst, being an esteemed "Emeritus Member" of the [Amethyst foundation](https://web.archive.org/web/20230423013649/https://amethyst.rs/team). (If you don't know what that means, it's basically like being Knighted, except without the problematic implications of serving the Royal Family, so it's actually far more prestigious in that regard).

I like to think the spirit of Amethyst is living on despite its ceased development, as Amethyst [directly inspired](https://github.com/bevyengine/bevy/blob/938d810766d34f1a300beb440273c3db1635ee5c/CREDITS.md?plain=1#L10) Bevy. Take a look at some [sample code](https://github.com/piedoom/j/blob/19ca304c12da31f4d29de080c6d7cda4156fdfdb/src/systems/movement.rs#L36C1-L57C2) from an Amethyst project I wrote 5 years ago:

```rs
#[derive(Default)]
pub struct MovementSystem {}

// I was much newer to Rust at this point, but
// I'm pretty sure this was idiomatic in Amethyst
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, Movement>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (movements, mut transforms, time): Self::SystemData) {
        for (movement, transform) in (&movements, &mut transforms).join() {
            // move at a constant speed in the direction
            transform.append_translation(movement.next(&time));
        }
    }
}
```

This all might look strangely familiar, even if you've never heard of Amethyst. We're defining a type `SystemData` that describes all the parameters we want to retrieve from our world, and then `join` it, sort of like a `Query`. In Bevy, this would look like the following:

```rs
fn movement_system(mut query: Query<(&Movement, &mut Transform)>, time: Res<Time>) {
    for (movement, mut transform) in query.iter_mut() {
        transform.append_translation(movement.next(&time));
    }
}
```

In essence, we're still defining `SystemData`, but defined in the parameters of our system. Visually, we can see how Bevy has improved the ergonomics of using an ECS over the past 5 years, and not just because I unfairly added two lines of comments to the first snippet. This is a very, very small facet of improvements, so while we're on the topic, I'd be remiss if I didn't talk about

## My favorite bevy improvements since v0.1

Subjective lists are fun! Do you agree? Do you disagree? Remember to leave a comment, and tap that notification bell.

### `system()`less systems

If you're newer to Bevy, this post's title might not mean much to you, but back in Bevy v0-something, we registered systems [like](https://github.com/bevyengine/bevy/blob/9afe196f1690a6a6e47bf67ac740b4edeffd97bd/examples/3d/3d_scene.rs#L4C5-L7C44):

```rs
App::build().add_startup_system(setup.system())
```

Amusingly, this, combined with developers' propensity to suffix their system functions respectively, led to system registrations that look like:

```rs
App::build().add_startup_system(birthday_system.system())
```

This is no more as of Bevy [0.6](https://bevyengine.org/news/bevy-0-6/#no-more-system) which requires only the function name - no `system()` call necessary.

> Bevy is fun to write, as its thoughtful design dovetails with Rust's type system and language features. After some practice, Bevy's API fades away into the background[^types].

### Better shaders

I'm really not a math guy. I still don't "get" Quaternions (and I'm not fully convinced anyone else does). By extension, I'm not much of a graphics guy. Adding a custom shader to Bevy when it first released involved touching render code and writing a thesis of boilerplate that I was never able to defend against rustc. With `0.6` Bevy moved to `wgsl` and simplified custom shader creation. 8 releases later, I finally managed to implement a star background shader.


I would still love to see more tutorials in this space, specifically for `wgsl`.

### Gizmos

Gizmos, introduced in [0.11](https://bevyengine.org/news/bevy-0-11/#gizmos), are a selection of "immediate mode" drawing primitives useful in debugging and prototyping. Gizmos are perfect for prototyping quick ideas while avoiding the added complexity of assets. I wrote a modest but respectable ~3,000 line project visualizing with only Gizmos before moving on to proper models.

{{mastodon(url="https://mastodon.social/@doomy/113095182170100748")}}

### Triggers and observers

I can't include the removal of `.system()` without mentioning any of the other *major* ECS improvements; the continual focus on ergonomics is a big reason I think Bevy has become a relatively popular engine.

To better understand what triggers do, let's first understand how to write a Bevy project _before_ triggers were introduced with regular events: Imagine we have a minefield. If one mine is triggered with a click, it causes an explosion with a defined radius. If any other mines are in range of that explosion, they're also triggered.

Using events, we can express this relationship like the following rusty pseudocode:

```rs
#[derive(Event)]
pub struct ExplodeEvent { entity: Entity }

fn handle_explode_events(mut cmd: Commands, events: EventReader<ExplodeEvent>, mut write_events: EventWriter<ExplodeEvent>) {
    for event in events.read() {
        let ExplodeEvent { entity } = event;
        // Do something with this entity like play an explosion noise, then despawn it
        cmd.entity(entity).despawn_recursive();
        // Find any entities within a set radius (here it is done ~magically~)
        let entities_to_explode = find_entities_in_blast_radius(entity);
        for entity_to_explode in entities_to_explode {
            write_events.send(ExplodeEvent { entity: entity_to_explode });
        }
    }
}
```

> For astute _observers_ this might _trigger_ a negative reaction, as this is an invalid system definition, requiring both mutable and immutable access to `ExplodeEvent` events. You'd actually need to use a `ParamSet` here, accumulate events to write in the event read loop, then send out events after reading.

Additionally (and more importantly) these events do not trigger on the same update. Any `entities_to_explode` are added to the event queue, and then `hande_explode_events` runs on the next update. This isn't ideal when scaled to hundreds of objects. By comparison, triggers execute in a single update, even when recursed.

Bevy's official examples [has code illustrating this exact minefield idea](https://bevyengine.org/examples/ecs-entity-component-system/observers/). Here's what the equivalent of the above looks like in our pseudocode:

```rs
#[derive(Event)]
pub struct ExplodeTrigger;

fn on_explode(trigger: Trigger<ExplodeTrigger>, mut cmd: Commands) {
    let entity = trigger.entity();
    // Do something with this entity like play an explosion noise, then despawn it
    cmd.entity(entity).despawn_recursive();
    // Find any entities within a set radius (here it is done ~magically~)
    let entities_to_explode = find_entities_in_blast_radius(entity);
    for entity_to_explode in entities_to_explode {
        cmd.trigger_targets(ExplodeTrigger, entity_to_explode);
    }
}
```

With triggers, all entities to explode are calculated in a single update, _and_ we don't have issues juggling event mutability.

---

# II. Looking back

## Organizing a bevy project for honor and success

After many projects, repetition is inevitable. As I am Very Into Rust, I'm also very into conventions and planning (also known as over-engineering). With experience, I've generalized a project organization that works well, and condensed it into a template repo, which I use for new projects. Its goal is not to be a feature-complete starter like [foxtrot](https://github.com/janhohenheim/foxtrot), but simply to nurture an understandable, scalable [project file structure](https://github.com/piedoom/sobevy).

### Very Important Crates

There are a few batteries included, but these are the crates I use in every project. Let's take a look at the default `Cargo.toml`:

```rs
[dependencies]
leafwing-input-manager = { git = "https://github.com/Leafwing-Studios/leafwing-input-manager/" }
bevy = { version = "0.14", features = ["serialize"], default-features = true }
bevy_common_assets = { version = "0.11", features = ["ron"] }
bevy_asset_loader = { version = "0.21", features = ["2d", "3d", "standard_dynamic_assets",] }
```

- `leafwing-input-manager` is a generalized way to get input over keyboards and gamepads. Folks, do we love to write generalized code? We love it. That guy over there loves it, I see them, they know.
- `bevy` is bevy.
- `bevy_common_assets` provides asset loaders for - would you believe this - common asset types like `ron`. In fact, I'm only using its `ron` feature (as `json` doesn't let you use trailing commas 😡😤)
- `bevy_asset_loader` implements all of the asset loading logic that you would have implemented yourself.

There's also a few commonly used but alternative crates ready to be unleashed, but commented out. They're all related to `egui`.

```toml
bevy_egui = "*"
bevy-egui-kbgp = "*"
egui_extras = { version = "*", features = ["file", "image"] }
image = { version = "*", features = ["png"] }
bevy-inspector-egui = "*"
```

The most important crate here is `bevy-inspector-egui`, which provides a powerful way to diagnose runtime issues, especially for people like me who think debugging is when you write `dbg!(..)`. We also make `bevy-egui-kbgp` available, which adds gamepad input support to `egui` menus. Until Bevy's own UI and editor gathers momentum, `egui` is the de-facto way to present UI in games. That's it though, just 3-ish crates by default. Outside of setting up `bevy_asset_loader`, this template doesn't really *do* much else. It's more about project organization.

> `egui`! It's a great library. However, the difficulty with styling, lack of gamepad support, and ever-increasing world data needed to render custom widgets renders it a less-than-optimal choice. Additionally, `egui` and Bevy's philosophies compete: there is a tendency to include all UI code in a single system to reduce complexity, leading to incredibly long and terse UI system functions. I'm looking forward to Bevy's own UI maturing providing a more constructive relationship between ECS and UI.

### Project organization

> My 6th grade math teacher had a poster that said "Organization is the key to success". So true! I also couldn't stay quiet during his class, which also taught me about detention. Although I don't consider myself a particularly organized person, my brain does have those peculiar deleterious set of genes innate to humans that make us clap our hands and jump up and down whenever we successfully organize things into groups.

> Here's another fact about my mysterious past: I used to be really into Ruby on Rails (and according to some Rust surveys, a lot you probably were, too). Rails is *all about* "convention over configuration". That quote itself was coined by Rails' creator, Hatsune Miku.

> MVC isn't exactly a sexy term. It's arguably sexless. But parts of MVC began to influence how I began to structure my Bevy projects, drawing inspiration particularly from strong separation of data and functionality. In Rails, the data bits are "models", and the functionality bits are "controllers". These are analogous to components and systems in Bevy. Rails has one more level of organization: controllers are not single functions, but a type with several inherited methods (e.g., "new", "create", "delete"). In Bevy, these are akin to `Plugin`s, which group systems into a single unit.

In my experience, it's rare that a system *doesn't* belong in a single `Plugin`. Grouping related systems can help ease the cognitive load of storing larger projects in (human) working memory. To achieve this, all systems in my projects are private[^utility], and only the containing `Plugin` is exposed, resulting in this directory structure:

```bash
src/
    components/
        mod.rs
        my_component.rs
    plugins/
        mod.rs
        my_thing.rs
    resources/
```

This simple setup works well for my projects and seems to grow manageably. So far.

#### Preludes

Bevy developers love preludes! `use bevy::prelude::*` is far easier than importing every item by name. So why not extend this pattern to my own projects and be my own consumer by adding a `pub use` statement within a `prelude` module in our `lib.rs`?

```rs
pub mod prelude {
    use super::*;
    #[allow(unused_imports)]
    pub use {components::*, error::*, plugins::*, resources::*, states::*, ui::*, util::*};
}
// Leave out `pub` for any types you don't want available in these modules.
```

With this pattern, our game's most important types are available with a single import of `use crate::prelude::*`.

> Note that I have a `states/` directory in my template repo, but I've yet to have an application need more than one state (and I don't even know if that's supported tbh). States _are_ just resources, so it's better to add a `state.rs` file in `resources/` instead.

### Naming

If it is necessary to convey type information in a type's name, it can be a signal that the project's organization may need improvement. `Controller` is a better type name than `ControllerComponent`; it is easier to type, clearer to read, and is already known to be a component by implementing the `Component` trait. If disambiguation is necessary, modules can be used. For example, event and trigger names are often similar to component names. When using the `prelude` method, a problem arises: What if we have an `Equip` component and an `Equip` event in the same namespace?


```rs
// Errors

pub mod prelude {
  use super::*;
  pub use { components::Equip, events::Equip };
}

pub mod components {
  pub struct Equip;
}

pub mod events {
  pub struct Equip;
}
```

It's tempting to add an "`Event`" to the end of the type's name, but this creates a new issue where `Event` types litter the project with no apparent grouping. We could *prefix* the type name with "Event" to search alphabetically for related types in our editor, but this *forces* us to be verbose, even in situations where it's unnecessary, and is downright awkward.

```rs
// Not the best solution

// Components are common so no suffix is added
pub struct Equip;
// Events get a suffix added to the type name
pub struct EquipEvent;
// pub struct EventEquip; // Would be better for searching for events, but looks weird
```

Instead of adding to the type name, we can leverage modules. Because component types are *extremely* common to use, they can stay in the `prelude` root. We'll move `event`s and `trigger`s to their respectively named modules available within `prelude`.

```rs
pub mod prelude {
  use super::*;
  // This is identical as our first example, but we're only exposing
  // the entire events module instead of specific items
  pub use { components::Equip, events };
}

pub mod components {
  pub struct Equip;
}

pub mod events {
  pub struct Equip;
}
```

```rs
// Use in the project
use crate::prelude::*;

fn my_system(equips: Query<&Equip>, events: EventReader<event::Equip>) {
    // No conflicts!
}
```

If in a particular file, components were completely unused, while events were used heavily, we could write `use crate::prelude::event::*`. This flexibility is a nice bonus to modules neatly separating our ever-growing list of types available in `prelude`.

> While we're crossing into subjective territory, I *like* namespacing by modules instead of adding extra info to the actual type name. That's what modules are there for to begin with, right? I *like* writing `event::Equip` instead of `Equip`, even if it weren't required due to aliasing another type in the prelude.

## Projects

Outside of a few projects, I've rarely pushed one to a "complete" playable state. I'm okay with that! I work on a product team of a tech company. It's nice to start creating without a clear goal. Ideating, finding creative solutions to problems, and promptly deleting those creative solutions because you've moved in a new direction isn't a good way to develop games, but it's a great way to learn.

Let's peruse the few dozen gibberish-named private repositories of ideas stashed on my GitH*b and see if I've actually learned anything.

## [bevy-puzzle](https://github.com/piedoom/bevy-puzzle)

It's a puzzle game with tetrominoes! [Simple as it is, I'm happy with how this turned out](https://piedoom.github.io/bevy-puzzle/). Sure, it may crash when you win because it tries to write to a highscores file that doesn't exist in the WASM build, but the several seconds of gameplay beforehand is fun!

This project highlights a recurring theme for me: I don't like 2D. Whenever I'm a few hours into a project using sprites, I can only think of how much easier everything would be in 3D space. I don't want to deal with sprite ordering in an isometric 2D game. We live in the future. We have access to more dimensions now, even the 2D is 3D[^dimensions].

{{mastodon(url="https://mastodon.social/@doomy/113095335789830890")}}

## [wordlrs](https://github.com/piedoom/wordlrs)

Wordle is a relatively simple game to implement, so [this project](https://piedoom.github.io/wordlrs/) furthered my working knowledge of egui with an achievable goal. A feature of this implementation is that it is language-independent. (If the language uses an alphabet. 对不起[^chinese]). While this sits among my rare playable projects, it's not at all interesting. Wordle doesn't particularly take full advantage of an ECS (but [Wordle 99](https://en.wikipedia.org/wiki/Tetris_99) absolutely would, someone get on that).

## [modsynth](https://github.com/piedoom/modsynth)

Not a Bevy project (nor a eurorack), modsynth is a naive model synthesis implementation (also known as "wave function collapse"). My main takeaway from this project is I am not good at math, and constraint solving is hard[^constraints].

I did, however, create a neat dynamically generated 3D scene using Bevy. It's exactly like "Lethal Company", except without any gameplay.

{{ mastodon(url="https://mastodon.social/@doomy/111756628711521099") }}

## [bevy-planty](https://github.com/piedoom/bevy-planty)

It's an [L-system](https://en.wikipedia.org/wiki/L-system) visualizer! L-systems use relatively simple instructions to generate complex structures, and can be used to model plants.

egui is great, but working on this project was an early encounter with its aforementioned pitfalls. There's more than a few broken buttons.

{{ mastodon(url="https://mastodon.social/@doomy/113109695733994318") }}

## [bevy blast ultra](https://github.com/piedoom/bevy_blast_ultra)

My solo Bevy jam #4 entry: a marble run game inspired by *Super Marble Blast Ultra* with levels designed in Blender using Blenvy. In retrospect, I have no idea how I pulled this off in a week. [I have an epic-style post talking about this some more](http://127.0.0.1:1111/bevy-jam-4/).

![Marble on rails in game](/bevy-jam-4/rails.png)

---

# III. Looking forward

## What I'm still waiting for

In my game, I have items, which are described as an `Asset` using ron.

```rs
/// A general in-game item
#[derive(Debug, Component, Clone, Reflect, Asset, Serialize, Deserialize)]
pub struct Item {
    /// The in-game name of this item
    pub name: String,
    /// The in-world mass of this item
    pub mass: f32,
    /// The space this item displaces in the [`Inventory`]
    pub size: usize,
    /// The worth of this item in `Credits`
    pub value: usize,
}
```

```rs
(
    name: "scrap metal",
    mass: 0.1,
    size: 1,
    value: 10,
)
```

Assets are perfect, because I can modify or extend my game with new items without recompilation.

Items aren't entirely useful without the ability to pick them up, so we'll also need an inventory:

```rs
// This is overly simplistic and not doing anything with our items `size` or `mass`, but it'll get the idea across, just wait.
#[derive(Debug, Clone, Component, Reflect)]
pub struct Inventory {
    /// Items and their count in the inventory
    items: HashMap<Handle<Item>, usize>,
}
```

`Inventory` is a `Component` that contains references to `Item`s in the form of a `Handle<Asset>`. This provides two obvious benefits: items can be hot-reloaded, and cloning handles is cheap compared to cloning an entire `Item`  fully, especially as items gain fields and additional complexity. This works well until we need to serialize.

`Handle`s are runtime specific, so while we can technically write a `Handle`'s ID to a file, it won't be valid the next run. Instead, `AssetPath`s are required to successfully serialize and deserialize. Bevy's [animation graph](https://docs.rs/bevy_animation/0.14.1/src/bevy_animation/graph.rs.html#166) does something similar to this, but it's not necessarily a fun or easy thing to implement in your own projects.

```rs
// From `bevy_animation` - https://docs.rs/bevy_animation/0.14.1/src/bevy_animation/graph.rs.html#166

/// A version of `Handle<AnimationClip>` suitable for serializing as an asset.
///
/// This replaces any handle that has a path with an [`AssetPath`]. Failing
/// that, the asset ID is serialized directly.
#[derive(Serialize, Deserialize)]
pub enum SerializedAnimationClip {
    /// Records an asset path.
    AssetPath(AssetPath<'static>),
    /// The fallback that records an asset ID.
    ///
    /// Because asset IDs can change, this should not be relied upon. Prefer to
    /// use asset paths where possible.
    AssetId(AssetId<AnimationClip>),
}
```

*This is just the data representation. Actually serializing and deserializing this isn't as straightforward. At least to me.*

The ability to "persist" asset handles is useful - especially with dependency resolution. Wouldn't it be nice if we could just _do_ that?

# bsn! bsn!

Bevy's proposed scene format is going to solve all of my life's problems. For one, it solves that `Handle<_>` issue by introducing `Construct`: a way to initialize components that need to access stuff from our ECS world. Now, we can serialize handles to an `AssetPath<_>`, and neatly deserialize back into a `Handle<_>` via access to the asset server. This can immediately simplify save strategies.

Now that we have a way to reference other assets, my idea of prefabs is possible. Let's go back to my top-down space game with the inventory as an example of why this is useful.

```bash
# Our assets directory
assets/
  items/
    laser.weapon.ron
    metal.item.ron
    battery.energy.ron
  models/
    crab.gltf
    ship.gltf
  creatures/
    crab.creature.ron
```

You can think of `creatures` as NPC definitions. If we load `crab.creature.ron`, we should be able to spawn it and its components. This is what the crab creature `ron` looks like:

```rs
(
    name: "crab",
    model: "models/crab.gltf",
    inventory: [
        ("items/laser.weapon.ron", 1),
        ("items/battery.energy.ron", 1),
        ("items/metal.item.ron", 3),
    ],
)
```

This is valid `ron`, so we could load this - but Bevy has no idea what these random path strings mean. We first need to map our asset paths to runtime handles. Luckily `[bevy_asset_loader](https://docs.rs/bevy_asset_loader/latest/bevy_asset_loader/)` does this for us. All we need to do is load our items and models, then load our creatures - probably by checking an `Added<Creature>` query and building the described entity manually. This works, though it's highly manual, prone to errors, annoyances with circular dependencies, and is a general bother to maintain and extend.

Bevy doesn't have a concept of a common collection of components on purpose (Bundles and Archetypes don't count). There is no concept of the "shape" of an entity known statically. Our "NPC" is nothing more than interactions between specific components: they are entities that have a `Transform` component, a `Controller` component for movement, and a `Thinker` component for directing `Controller` movement. **Nothing in the type system describes that specific relationship of components** - and it shouldn't. Bevy's API encourages systems to use the least amount of components necessary, which results in *general* code that adapts with emergent behavior as systems and components are added.

But, our `ron` data description goes against this philosophy and limits flexibility 😢. What if we wanted to add a unique `BossName` component? We'd have to add a whole new field to the `creature` struct to support this optional `BossName` component, which means recompilation is required. It also inevitably grows the entity description struct into a gigantic mess of `Option<Component>` fields.

With `bsn`, we can instead define almost *everything* in our game as a Bevy scene, with any arbitrary components, including our `BossName` or anything else, allowing us to build significant game content without needing to venture into the codebase.

This is similar to Bevy's `DynamicEntity` (used in `DynamicScene`), except that we can now add and save components with world context, meaning we can serialize a `Handle<_>` to an `AssetPath<'_>` and back into a `Handle<_>`.

[Check out the whole thread on GitHub for more details on Bevy's new scene system](https://github.com/bevyengine/bevy/discussions/14437).

## Closing thoughts

Organization is not easy. Bevy hitting 4 years, all the while gaining momentum, is beyond encouraging. Bevy community members are some of the most engaging, friendly, and helpful people I've met online. It's special to share a space where there's little profit motive - just creative people making cool stuff that inspires them.

---

### Footnotes

[^ui]: Including some of the earliest work on a fully-Rust UI toolkit named [conrod](https://github.com/PistonDevelopers/conrod). I can't overstate how cool it was to be able to write a GUI application, completely in Rust, 5 years ago.

[^piston]: There's no indication that Piston is EOL like Amethyst, and there are some commits this year, but it looks to be in maintence mode.

[^types]: You know you're writing good rust when you have to include `#[allow(clippy::too_many_arguments)]` or `#[allow(clippy::type_complexity)]`

[^utility]: Sort of. Generic or error handling systems especially may be used across many plugins. I add these special cases to a `utility` module public to the crate.

[^dimensions]: When I work in 2D, I feel like I'm slowly walking down a big hill, wishing I remembered my Heelys so I could just slide to the bottom.

[^chinese]: Some argue that Chinese has an alphabet, it's just thousands and thousands of characters long. In any case it cannot be wordled (but can be [pinyindle'd](https://www.pinyindle.com))

[^constraints]: I may have had an easier time if I referenced more than [a single YouTube video on repeat for several hours](https://www.youtube.com/watch?v=A2ODauA1a0M). (This entire experience also revealed to me just how criminally underappreciated Paul Merrell is for his research in this area.)
