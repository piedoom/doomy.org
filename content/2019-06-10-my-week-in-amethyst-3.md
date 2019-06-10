+++
title = "My Week In Amethyst 3 - 3D & Prefabs"
author = "doomy"
template = "page.html"
date = 2019-06-10T20:05:23.927Z
description = " "

[taxonomies]
categories = []

[extra]
tags = []
+++

This week, I worked on something new for a change of pace. 

<video width="100%" controls>
    <source src="/uploads/fixed.mp4" type="video/mp4">
    Your browser does not support the video tag.
</video>

# New Things

Yeah, I know. *Another* new thing. It's fine though, I'm not too attached. This is all about learning, and I'm going to continue to try.

I wanted to create a asteroids-style movement game based roughly on the base building mechanics of my favorite MMO growing up, [Star Sonata](https://www.starsonata.com/).

This game has different ships, which can be customized with different engines, radars, weapons, and more. Sounds like a great opportunity to learn about prefabs, right?

# Prefabs

Well, I finally did it! I delved into prefabs. They are unfortunately one of the more confusing and complex parts of Amethyst. Once you get the hang of them, though, they aren't too bad. However, this area is definitely one where Amethyst can improve (and it should improve once [Atelier Assets](https://github.com/amethyst/atelier-assets) lands).

## A short explaination
Prefab data is just a struct with a ton of optional fields. This is really the only important part of prefabs, the rest is (again, unfortunately) boilerplate.

An example prefab definition: 

```rs
#[derive(Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct EntityPrefabData {
    pub name: Option<Named>,
    mesh: Option<MeshPrefab<GenMeshVertex>>,
    material: Option<MaterialPrefab>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
    camera: Option<CameraPrefab>,
    transform: Option<Transform>,
    light: Option<LightPrefab>,
    player: Option<c::Player>,
    controller: Option<c::Controller>,
}
```

So why the options? Well, currently there is a limitation where multiple prefab definitions in a `ron` file must have homogenous data (though it's gotten [some improvements](https://github.com/amethyst/amethyst/pull/1625) since I started).

## Making prefabs useful

I heavily stole from Amethyst's showcase project, [Evoli](https://github.com/amethyst/evoli) - specifically its [prefab resource](https://github.com/amethyst/evoli/blob/master/src/resources/prefabs.rs). This code inserts all prefabs in a given folder into a hashmap, and then inserts itself as a resource. That lets us in turn build more data-driven games.

# Movement

My movement is... O.K. It turns out that Asteroids-style thrust movement isn't as easy as it would seem, especially when all resources on the internet talk about how to do so using a physics system, which I wanted to avoid for now.

However, I came up with a decent interim solution. 

```rs
// rotate based on unit points
transform.append_rotation_z_axis(
    // This will orient the rotation direction correctly
    controller.rotation_control *
    // Multiply by our turn speed, which is just a multiplier.
    Float::from(controller.turn_speed) *
    // Finally, multiply everything by our delta to keep consistent across framerates
    Float::from(time.delta_seconds()),
);

// Set thrust and velocity

// this gets our facing direction
let rotation = transform.isometry().inverse().rotation.to_homogeneous();
let direction = Unit::new_unchecked(Vector3::new(rotation.row(UP)[0], rotation.row(UP)[1], Float::from(0.)));

// If our input is 0, we're not changing our velocity.
if controller.thrust_control != Float::from(0.) {
    controller.velocity = Unit::new_normalize(
        controller.velocity.as_ref() + direction.scale(controller.thrust_control * controller.acceleration));
}


// Finally, actually transform, multiplying by our max speed and delta
transform.prepend_translation(
    controller
        .velocity
        .scale(controller.max_speed * Float::from(time.delta_seconds())),
);
```

You can see the whole thing [in context here](https://github.com/piedoom/s/blob/master/src/systems/controller.rs). Please note this is by no means a very good solution, and I've already run into some issues with it cutting off important data.

# 3D Rendering

This part is actually pretty easy. Again, I stole a bunch of code from the Rendy example, and then modified it to only provide the PBR pass (or so I think). You can see what I have so far at the top! It's rendering a plane, so it doesn't look 3D, but trust me - it is. 

# Next Week

Next, I'm hoping to do more with modeling practice, as well as figuring out prefabs better. See you there!