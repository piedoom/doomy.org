+++
title = "Getting the \"on-screen\" coordinates of a transform in Bevy when it's not actually on screen "
author = "doomy"
template = "page.html"
date = 2021-09-27T00:21:30.437Z
description = "How to get the pixel coordinates of a position without relying on the position to be in camera in Bevy v0.5. This works only with OrthographicProjection."

[extra]
tags = ["rust"]
+++
Bevy provides the [`world_to_screen`](https://github.com/bevyengine/bevy/blob/9d453530fac9202691c797184ad0b220b2ea37b3/crates/bevy_render/src/camera/camera.rs#L46) method as part of the `Camera` component. It looks something like this:

```rs
fn my_bevy_system(camera: Query<(&Camera, &GlobalTransform)>, windows: Res<Windows>) {
    let world_position_to_find = Vec3::new(1.0, 2.0, 3.0);
    camera
        .single()
        .map(|(camera, camera_transform)| {
            let screen_position =
                camera.world_to_screen(&windows, camera_transform, world_position_to_find);
            if let Some(coords) = screen_position {
                // do something here!
            }
        })
        .ok();
}
```

`world_to_screen` returns an `Option`, because if the `world_position_to_find` is not within the camera's view, its screen coordinates cannot be calculated. 

## Getting the screen coordinates anyways, even off-camera

So, how would you get the relative screen position of an off-camera transform? If you are using an orthographic projection, you can instead use the scale and relative offset to calculate the screen position. The following closure demonstrates what that might look like.

```rs
let pos_to_pixels = |pos: Pos2| -> Vec3 {
    let scalar = 1f32 / projection.scale;
    let unit_offset = trans - Vec3::new(pos.x, pos.y, 0f32);
    unit_offset * scalar
};
```

Closures are great in bevy because they allow us to define some reusable code without needing to pass in a bunch of `Query`s or resources.

If you need it in a function, you can use: 

```rs
fn pos_to_pixels(
    pos: Pos2,
    projection: &OrthographicProjection,
    camera_transform: &GlobalTransform,
) -> Vec3 {
    let scalar = 1f32 / projection.scale;
    let unit_offset = camera_transform.translation - Vec3::new(pos.x, pos.y, 0f32);
    unit_offset * scalar
}
 ```

This will get the screen coordinates, even if they exist off of the camera.