+++
title = "Loading folders in Bevy 0.6 web (sort of)"
author = "doomy" 
date = 2022-01-13 
description = "Loading folders isn't currently supported on wasm32 targets for bevy, but it can still kind of be done."
draft = true

[taxonomies] 
tags = ["rust", "gamedev"]
+++

[Bevy 0.6 released with support for targeting web](https://bevyengine.org/news/bevy-0-6/#webgl2-support) without the need for
any extra configuration.  While upgrading my game from a previous release to
take advantage of web, I ran into an issue where I could not use the
[`load_folder`](https://docs.rs/bevy/0.6.0/bevy/asset/struct.AssetServer.html#method.load_folder) method.

{% discord( name="doomy",
    src="https://discord.com/channels/691052431525675048/742884593551802431/929666632790470667",
    img="https://cdn.discordapp.com/avatars/210437037751402507/26f24a9ca99ba8a2638d6f6abedf42dc.webp?size=100",
    date="01/09/2022") 
%}
From what I can see, load_folder is not supported with wasm currently. I was
wondering if there is an alternative I can use that would work and would allow
specifying directories? {% end %}

Take for instance, the following line from the [official asset loading
example](https://github.com/bevyengine/bevy/blob/b724a0f586e6186f2a6ce4eb7903be0e340649e9/examples/asset/asset_loading.rs#L41):

```rs
let _scenes: Vec<HandleUntyped> = asset_server.load_folder("models/monkey").unwrap();
```

Assuming we have some files in the `/assets/models/monkey` directory, this
method of loading works fine on native platforms. However, when compiling for
WASM, you'll find an error stating that `load_folder` is unsupported.

In my case, I really liked the flexibility `load_folder` gives me. I have a
moderately complex setup of assets that belong to themes, and being able to
define everything with the asset file helps speed things up.

The problem to figure out is: how can we make a new `load_folder` function that
loads single files in WASM and retains loading a folder dynamically on native.

## `build.rs`

While there's a lot of *much* better ways to solve this problem, I just wanted
something **now**, as a better asset pipeline is in the near future.

I ended up solving this issue by creating a manifest of all asset files created
on compilation that can be referenced on the web as single files. 

```rs
use serde::{self, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Path to the manifest that will be created (relative to Cargo.toml)
const PATH: &str = "assets.manifest";

fn main() {
    let assets_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets");

    // Here we list all the folders that assets are loaded within. It's
    // important to note that this implementation does not account for nested
    // directories. If you are looking for that, maybe try the `walk` crate. In
    // my case, I have a whole ton of directories (campaigns, fonts, maps, etc.)
    // If you're looking to list directories automatically, you could build this
    // hashmap with a folder read. I specifically wanted to include only some
    // assets in this build.
    let mut assets: HashMap<String, Vec<String>> = [
        ("my_assets".into(), vec![]),
        // ... any other directories here
    ]
    .iter()
    .cloned()
    .collect();

    // Mutate the hashmap in place with contents from the asset directory
    assets.iter_mut().for_each(|(folder, files)| {
        // get every asset directory and map the contents
        assets_dir
            .join(folder)
            .read_dir()
            .unwrap()
            .for_each(|asset| {
                // map to iterator and then map to success to get the inner item
                let asset = asset.unwrap();
                files.push(asset.path().file_name().unwrap().to_str().unwrap().into());
            });
    });

    // Put the assets within a struct just so it saves in RON a bit nicer within
    // a struct. You can also probably just do some funky `format!()` stuff here
    let assets = Container(assets);
    File::create(assets_dir.join(PATH)).unwrap();

    // Save the resulting RON manifest in the assets folder. Depending on what
    // you want you might want to put this elsewhere that isn't committed to
    // VCS, like target
    if let Ok(text) = ron::to_string(&assets) {
        let mut file = File::create(assets_dir.join(PATH)).unwrap();
        file.write_all(text.as_bytes()).ok();
    }
}

/// Container to match format on game side
#[derive(Serialize)]
struct Container(pub HashMap<String, Vec<String>>);
```

So, what does it do? Every time the project compiles, this code stashes all the
links required to load in a RON manifest file within the `assets` directory.
Here's an example of what mine looks like: 

```json
(
    {
        "campaigns":["default.campaign"],
        "saves":[
            // Oops! it got a file that isn't a save...
            // Probably fine but uh... maybe do a check?
            ".gitkeep", 
            "1639263227.save",
            "1639350956.save",
            "1639352685.save",
            "1641165412.save",
            "1641712186.save"
        ],
        "themes": ["3d.theme","default.theme","rust.theme"],
        "maps": ["circle.map","default.map","dot.map","ibeam.map","skew.map"],
        "my_assets": ["my_asset_1.file", "my_asset_2.file"],
        // These are folders full of stuff
        "sprites": ["3d","default","rust"]
    }
)
```

We can use this file when on the web to make a web-compatible `load_folder`
implementation. 

```rs
#[cfg(not(target_arch = "wasm32"))]
fn load_folder(assets: &AssetServer, folder: &str) -> Vec<HandleUntyped> {
    assets.load_folder(folder).expect("Could not load")
}

#[cfg(target_arch = "wasm32")]
fn load_folder(
    assets: &AssetServer,
    folder: &str,
    manifests: Res<Assets<AssetManifest>>,
) -> Vec<HandleUntyped> {
    let manifest = &manifests.iter().next().unwrap().1 .0;
    manifest
        .get(folder)
        .unwrap()
        .iter()
        .map(|path| assets.load_untyped(PathBuf::from(format!("{}/{}", folder, path))))
        .collect()
}
```

For non-WASM code, `assets.load_folder(..)` will be called as usual. However,
when building for web, the second function compiles instead which makes use of
the manifest file. It loads assets one-by-one in a loop according to the paths
supplied by the manifest. 

However, we will need to define a manifest asset, and load it first for this
helper function to work. For this, we'll make load of a game state and
`SystemSet`s. 

```rs
use std::path::PathBuf;
use bevy::{asset::LoadState, prelude::*};

use bevy::{reflect::TypeUuid, utils::HashMap};

#[derive(serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Default, Debug, Clone, Eq)]
#[uuid = "fccfcc12-3456-4fa8-adc4-78c5822269f8"]
struct AssetManifest(pub HashMap<String, Vec<String>>);

struct MyAssetPlugin;

impl Plugin for MyAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            // Keeps track of handles loaded
            .init_resource::<PreloadingAssets>()
            .init_resource::<Stage>()
            .add_asset::<AssetManifest>()
            // I'm making use of the ron_asset_plugin crate for easier loading,
            // but you can manually implement this asset too.
            .add_plugin(RonAssetPlugin::<AssetManifest>::new(&["manifest"]))
            .add_system_set(
                SystemSet::on_enter(GameState::load()).with_system(load_manifest_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::load()).with_system(load_assets_system),
            );
    }
}

/// Our loading will take multiple stages since some assets have dependencies 
/// to make developing easier.
/// We can specify the current stage here and keep it in state. This is a bit
/// nicer than defining a ton of other state variants, as you'll find that
/// loading stuff in this hacky way will always require a couple more loops
/// thank you thought.
#[derive(Default)]
pub struct Stage(usize);

impl Stage {
    #[inline(always)]
    pub fn current(&self) -> usize {
        self.0
    }
    #[inline(always)]
    pub fn next(&mut self) {
        self.0 += 1
    }
}

// Loads the manifest file first so paths to other assets can be specified
fn load_manifest_system(mut loading: ResMut<PreloadingAssets>, assets: Res<AssetServer>) {
    loading.0.push(assets.load_untyped("assets.manifest"));
}

// Loads the rest of the assets. Depends on the `AssetManifest` being loaded
fn load_assets_system(
    mut state: ResMut<State<GameState>>,
    mut loading: ResMut<PreloadingAssets>,
    mut stage: ResMut<Stage>,
    assets: Res<AssetServer>,
    // We access manifests, but only after they are done loading. Also,
    // we'll only need this for WASM builds (but your use case may differ).
    #[cfg(target_arch = "wasm32")] manifests: Res<Assets<AssetManifest>>,
) {
    // If no handles are loading anymore, we are done loading.
    let done_loading = loading
        .0
        .iter()
        .filter(|h| assets.get_load_state(*h) == LoadState::Loading)
        .count()
        == 0;

    // ...

    if done_loading {
        match stage.current() {
            0 => {
                // Done loading the actual manifest file. 
                // We increment our stage to load more.
                stage.next();
            }
            1 => {
                // Able to load folders!
                let mut my_asset_handles = load_folder(
                    &assets,
                    "my_assets",
                    #[cfg(target_arch = "wasm32")]
                    &manifests,
                );
                loading.0.append(&mut my_asset_handles);

                // ...

                stage.next();
            }
            _ => {
                // Watch for changes (if desired) and set state to next
                assets
                    .watch_for_changes()
                    .expect("could not watch for changes");
                // Transition states to the menu (or whatever is your post-loading state)
                state.set(GameState::Menu).ok();
            }
        }
    }
}
```

It's a little clunky to just call a function instead of a method on
`AssetServer`, so you *could* technically do something nice with extension
traits, but that seems like too much work for what is essentially a hack.
