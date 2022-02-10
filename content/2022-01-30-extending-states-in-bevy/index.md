+++
title = "Extending states in Bevy"
author = "doomy" 
template = "page.html" 
date = 2022-01-30
description = "Storing data within Bevy states and using SystemSets"
draft = false

[taxonomies] 
tags = ["rust", "gamedev"]
+++

> This was written for Bevy v0.6

In Bevy, we use [`State`s](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.State.html) to organize our game's functionality. We can use them to order when and where systems run.

## Understanding states

This following an example of a typical usage of states. First, we load assets, and then transition to the main game state when asset loading has completed.

```rs
use bevy::{asset::LoadState, prelude::*};

fn main() {
    App::new()
        .add_state(GameState::Load)
        .add_system_set(SystemSet::on_enter(GameState::Load).with_system(load_assets_system))
        .add_system_set(SystemSet::on_update(GameState::Load).with_system(check_loaded_system))
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(do_game_stuff_system))
        .run();
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum GameState {
    Load,
    Game,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

fn load_assets_system(mut loading: ResMut<PreloadingAssets>, assets: Res<AssetServer>) {
    loading.0.extend(assets.load_folder("my_folder").unwrap());
}

fn check_loaded_system(
    mut state: ResMut<State<GameState>>,
    loading: ResMut<PreloadingAssets>,
    assets: Res<AssetServer>,
) {
    let finished_loading = loading
        .0
        .iter()
        .any(|handle| assets.get_load_state(handle) == LoadState::Loading);

    if finished_loading {
        state.replace(GameState::Game).ok();
    }
}

fn do_game_stuff_system() {
    // ...
}
```

So, what's going on here? We define a `GameState` enum with two variants, `Load`, and `Game`. We load assets in our `Load` state (wow!) and then check for them to finish loading. When all assets are fully loaded, we change the state variant to `Game` and `do_game_stuff_system` where we can interact with those newly loaded assets. 

There's a few important things to look out for in this setup. Notice the requirements of a Bevy state: it must be `Default, Debug, Clone, PartialEq, Eq, Hash`. 

Secondly, note the way systems are added to our game. Instead of using `add_system` directly, we instead use `add_system_set` to define when the system should run in relation to our state. For example, `SystemSet::on_enter(GameState::Load).with_system(load_assets_system)` tells our game to run `load_assets_system` once when our state begins with (or has transitioned to) `Load`. `SystemSet::on_update(GameState::Load).with_system(check_loaded_system)` tells our game to run `check_loaded_system` on every *update* of the `Load` state, instead of just once. This continues until the state changes to `Game`.

## The limitation

The above is excellent for getting started, but is limited due to our `GameState` being unable to contain variant data.

To understand this further, we need to add some more context. Let's assume a few things about our example game. Lets say it is a puzzle game with a `Load`, `Game`, and `PostGame` state. 

We need to keep track of some game settings. We might store our settings as a `Resource`. So, a simplified timline of events our game executes might look like this: 

Load assets ➡️ Initialize resources ➡️ Transition to main game state ➡️ Play game ➡️ Transition to post Game state ➡️ Click replay ➡️ Re-initialize resource ➡️ Transition back to game state to replay

The not-so-great part of this is that we must remember to manually initialize and reinitialize certain resources for a state. While not an issue for our contrived example with just a score, this *will* become much more difficult to track once your game becomes more complex. One option is to use `on_exit` `SystemSet`s to reset certain resources. While useful, what if we could tie that data directly to our states?

## Making states more powerful

We can use enum variants to solve this. Our new `GameState` and associated data will look like this:

```rs
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Settings {
    difficulty: Difficulty,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum GameState {
    Load,
    Game(Settings),
    PostGame(Settings),
}
```

You might notice this causes a few issues. Our 
`SystemState` declarations no longer make any sense. We don't want to run our system only when the score is a specific value, and rustc requires us to specify variant data.

```rs
// ...
.add_system_set(SystemSet::on_enter(GameState::Load(Score(1))).with_system(load_assets_system))
// ...
```

However, we only want to check the [discriminant](https://doc.rust-lang.org/std/mem/fn.discriminant.html) when it comes to running systems. To solve this, we can create a custom `PartialEq` implementation. 

```rs
impl PartialEq for GameState {
    /// Set a custom equality method that only compares the enum variant,
    /// ignoring any attached data.
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
```

That, combined with our state will now look like this:

```rs
use bevy::{asset::LoadState, prelude::*};

fn main() {
    App::new()
        .add_state(GameState::Load)
        .add_system_set(
            SystemSet::on_enter(GameState::load()).with_system(load_assets_system))
        .add_system_set(
            SystemSet::on_update(GameState::load()).with_system(check_loaded_system))
        .add_system_set(
            SystemSet::on_update(GameState::game()).with_system(do_game_stuff_system))
        .run();
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Settings {
    difficulty: Difficulty,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Normal,
        }
    }
}

// We have to derive these as well but there's probably a way
// to get out of that since we aren't actually comparing this data.
// Not sure! Someone let me know.
#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Eq, Clone)]
enum GameState {
    Load,
    Game(Settings),
    PostGame(Settings),
}

impl PartialEq for GameState {
    /// Set a custom equality method that only compares the enum variant,
    /// ignoring any attached data.
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

// We also need to manually implement Hash since we did for PartialEq
// Thanks to DJMcBonk on Discord for helping ensure this was impl'd correctly
// See: https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq
impl Hash for GameState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl GameState {
    #[inline(always)]
    fn load() -> Self {
        Self::Load
    }

    #[inline(always)]
    fn game() -> Self {
        // The value here isn't actually ever checked due to
        // our custom PartialEq, but we need to provide it nonetheless
        Self::Game(Settings::default())
    }

    #[inline(always)]
    fn post_game() -> Self {
        Self::PostGame(Settings::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

fn load_assets_system(mut loading: ResMut<PreloadingAssets>, assets: Res<AssetServer>) {
    loading.0.extend(assets.load_folder("my_folder").unwrap());
}

fn check_loaded_system(
    mut state: ResMut<State<GameState>>,
    loading: ResMut<PreloadingAssets>,
    assets: Res<AssetServer>,
) {
    let finished_loading = loading
        .0
        .iter()
        .any(|handle| assets.get_load_state(handle) == LoadState::Loading);

    if finished_loading {
        // Start our game and initialize the difficulty as normal
        state.replace(GameState::Game(Default::default())).ok();
    }
}

fn do_game_stuff_system(state: Res<State<GameState>>) {
    // to access our state, we need to destructure it.
    // Note that because this system **only** runs on this state,
    // it should always succeed. If you wanted, you could match this
    // with the other arm being unreachable!().
    if let GameState::Game(settings) = state.current() {
        // We can now access settings freely!
    }
}
```

## Limitations of this method

There's a few tradeoffs while using states like this. First, states aren't mutable. You can't do something like `if let GameState::Game(mut settings) = state.current_mut() {}`, as states can only be pushed, replaced, or removed using the `State` resource. This means that you would still need resources for tracking data that is mutable within a single state, such as a game score. 

As with anything, it's just another tool.