+++
title = "Getting the cursor world position in a Bevy 3D game with Rapier"
author = "doomy"
template = "page.html"
date = 2021-04-25T05:45:53.416Z
description = "Getting the cursor world position in a Bevy 3D game with Rapier"

[taxonomies]
categories = ["rust", "bevy"]

[extra]
tags = ["game"]
+++
There's a few ways already out there to capture the cursor in [Bevy](https://bevyengine.org) . If you're making a game in 2D space with an orthographic camera, there's some [code in the Bevy Cheat Book](https://bevy-cheatbook.github.io/cookbook/cursor2world.html#2d-games). If you're creating a game in 3D space, and need to select entities, there is [bevy_mod_picking](https://lib.rs/crates/bevy_mod_picking).

I needed the world coordinates of my mouse, not a picker. I'm also using [Rapier](https://rapier.rs). The prospect using Rapier's physics system - instead of including another library - looked compelling.

## Caveats

This method might not work depending on how your game is set up. I've only tested in a 3D perspective camera world. Additionally, the cursor position will only update when moved on the client-side. This can be an issue if the camera moves. The world position of the cursor may change without the device moving, but no update event will send.

## Ray extension traits

To get the mouse position, we need to cast a ray from our camera.

I'm using [extension traits](https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html) to add some new functionality to Rapier's `Ray` type. I adapted some code from a [Bevy PR](https://github.com/bevyengine/bevy/pull/615/files#diff-b8d1b19c39cd5204a806524463a0dd17a744079b4ffae0819b9056d6eb718533R11) by [MarekLg](https://github.com/bevyengine/bevy/pull/615#issue-496846792) to do this. (Thanks!)

    pub trait RayExt {
        fn from_window(window: &Window, camera: &Camera, camera_transform: &GlobalTransform) -> Self;
        fn from_mouse_position(
            mouse_position: &Vec2,
            window: &Window,
            camera: &Camera,
            camera_transform: &GlobalTransform,
        ) -> Self;
    }

    impl RayExt for Ray {
        fn from_window(window: &Window, camera: &Camera, camera_transform: &GlobalTransform) -> Self {
            Self::from_mouse_position(
                &window.cursor_position().unwrap(),
                window,
                camera,
                camera_transform,
            )
        }

        fn from_mouse_position(
            mouse_position: &Vec2,
            window: &Window,
            camera: &Camera,
            camera_transform: &GlobalTransform,
        ) -> Self {
            if window.id() != camera.window {
                panic!("Generating Ray from Camera with wrong Window");
            }

            let x = 2.0 * (mouse_position.x / window.width() as f32) - 1.0;
            let y = 2.0 * (mouse_position.y / window.height() as f32) - 1.0;

            let camera_inverse_matrix =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            let near = camera_inverse_matrix * Vec3::new(x, y, 0.0).extend(1.0);
            let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

            let near = near.truncate() / near.w;
            let far = far.truncate() / far.w;
            let dir: Vec3 = far - near;
            let origin = Point3::new(near.x, near.y, near.z);

            Self {
                origin,
                dir: dir.to_vector3(),
            }
        }
    }

Now we have some methods (mainly `from_window`) that we can call to get a rapier `Ray`. We can use this to check for collisions.

While optional, I took inspiration from [some code](https://github.com/aevyrie/bevy_mod_raycast/blob/8b2ee7d015b9bb886684d7ad7796e404944bd5dd/src/primitives.rs#L95) in [`bevy_mod_raycast`](https://lib.rs/crates/bevy_mod_raycast) to create a Ray creation helper function.

    pub fn screen_to_world(
        windows: &Res<Windows>,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Ray {
        let window = windows
            .get(camera.window)
            .unwrap_or_else(|| panic!("WindowId {} does not exist", camera.window));
        Ray::from_window(window, camera, camera_transform);
    }

## Getting the cursor

In my case, I needed to get the cursor's position projected on a 2D plane (parallel to the camera's view). To do this in Rapier, we can use a [`HalfSpace`](https://docs.rs/rapier3d/0.8.0/rapier3d/geometry/struct.HalfSpace.html).

I also wanted to save these coordinates in a resource to access in my other systems.

    #[derive(Default)]
    pub struct CursorPosition {
        pub screen: Vec2,
        pub world: Vec3,
    }

    pub fn update_cursor_position(
        mut cursor_moved: EventReader<CursorMoved>,
        camera: Query<(&Camera, &GlobalTransform)>,
        windows: Res<Windows>,
        mut cursor_position: ResMut<CursorPosition>,
    ) {
        if let Some(screen_position) = cursor_moved.iter().last().map(|f| f.position) {
            if let Ok((camera, camera_transform)) = camera.single() {
                let ray = screen_to_world(&windows, camera, camera_transform);
                let plane = HalfSpace::new(Unit::new_normalize(Vec3::Z.to_vector3()));
                if let Some(toi) = plane.cast_local_ray(&ray, Real::MAX, false) {
                    let r = ray.point_at(toi);
                    cursor_position.screen = screen_position;
                    cursor_position.world = Vec3::new(r.x, r.y, r.z);
                }
            }
        }
    }

## Going beyond

We don't have to test on a `HalfSpace`, we can use any shape with any transform. (If you are applying a transform, use `.cast_ray` instead of `.cast_local_ray`.) We can also use this rapier `Ray` to test for collisions on entities, and create a simple picker.