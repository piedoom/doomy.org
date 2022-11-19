+++
title = "Rust VST part 4: creating a GUI"
author = "doomy" 
date = 2022-02-27
description = "Creating an audio plugin with the Rust programming language: Part 4"
draft = true

[taxonomies] 
tags = ["rust", "audio"]

[extra]
prev="2022-02-23-creating-an-audio-plugin-with-rust-3/index.md"
hidden=true
+++

So far, we've been controlling our plugin through the dials and knobs our host provides. However, most commercial audio plugins also include a custom GUI. In this chapter, we will create a simple user interface with `egui` and `baseview` related crates.

## Setting up logging

Before we start, it may be useful to enable logging. VST plugins don't do anything with print statements (e.g. `println!()`). Instead, we must log messages to a file. 

Because of Rust and `fundsp` designs, the risk of introducing a panic within our process block is slim. Yet, different systems and hosts can react unpredictably, and logs are a useful diagnostic. To capture logs, we'll be adding a few more crates:

{{ filename(name="src/Cargo.toml") }}
```ini
[dependencies]
# ..
log = "0.4"
simplelog = "0.11"
log-panics = "2"
```

In `lib.rs`, implement the `init` method of the `Plugin` trait to set up file logging:

{{ filename(name="src/lib.rs") }}
```rs
impl Plugin for Synthy {
    // ...
    fn init(&mut self) {
        // Set up logs, adapted from code from DGriffin91
        // MIT: https://github.com/DGriffin91/egui_baseview_test_vst2/blob/main/LICENSE
        let Info {
            name,
            version,
            unique_id,
            ..
        } = self.get_info();
        let home = dirs::home_dir().unwrap().join("tmp");
        let id_string = format!("{name}-{version}-{unique_id}-log.txt");
        let log_file = std::fs::File::create(home.join(id_string)).unwrap();
        let log_config = ::simplelog::ConfigBuilder::new()
            .set_time_to_local(true)
            .build();
        simplelog::WriteLogger::init(simplelog::LevelFilter::Info, log_config, log_file).ok();
        log_panics::init();
        log::info!("init");
    }
    // ...
}
```

This code will set up a file using the plugin's name, version, and ID to which to write logs. `log_panics` lets us capture any panic messages, too. We find the current home directory using `dirs`, and log to a subfolder named `tmp`, as simply adding a log adjacent to the plugin binary may result in issues [^program-files]. To test that this works, `cargo build --release` like usual, and load up the plugin in your host. If you open `~/tmp/synthy-{bunchofnumbers}-log.txt`, it should read something like:

```bash
05:36:50 [INFO] init
05:36:50 [INFO] Host is asking if plugin can: ReceiveMidiEvent.
05:36:50 [INFO] Host is asking if plugin can: SendMidiEvent.
05:36:50 [INFO] Host is asking if plugin can: ReceiveSysExEvent.
05:36:50 [INFO] Host is asking if plugin can: Other("MPE").
```

Everything looks good (except for some `CanDo`s that we haven't handled. But that's fine.) We now have logging, and can check if the plugin has issues loading or is crashing.

## Getting a UI to show
 
We need to bring in a few more crates to get our UI working. Note the specific required `rev`, which matches the version used in `egui-baseview`.

{{ filename(name="Cargo.toml") }}
```toml
[dependencies]
# ...
egui = "0.15"
egui-baseview = {git = "https://github.com/BillyDM/egui-baseview" }
baseview = { git = "https://github.com/RustAudio/baseview.git", rev = "f6e99e9aa6f5aeb6b721cb05e4d882a51d995909" }
raw-window-handle = "0.3"
# ...
```

> Don't worry if you're not 100% sure what's going on in the next few code blocks. This is plumbing to get our UI to show up. Once it's set up, we won't need to touch a lot of this code again.

Create a new file called `editor.rs`. Add a `mod editor;` line somewhere in `lib.rs` to include the new `editor` module. Initialize the `editor.rs` file with the following code [^credits]. Note that our code will not compile for the next few code blocks.

{{ filename(name="src/editor.rs") }}
```rs
use crate::Parameters;
use baseview::*;
use egui::*;
use egui_baseview::*;
use std::sync::Arc;
use vst::{editor::Editor, plugin::PluginParameters};

// ------------------ //
// 1. Setting UI size //
// ------------------ //
const WINDOW_WIDTH: usize = 256;
const WINDOW_HEIGHT: usize = 256;

// --------------------------------- //
// 2. Creating `PluginEditor` struct //
// --------------------------------- //
pub struct PluginEditor {
    pub params: Arc<Parameters>,
    pub window_handle: Option<WindowParent>,
    pub is_open: bool,
}

// ------------------------ //
// 3. Implementing `Editor` //
// ------------------------ //
impl Editor for PluginEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
        if let Some(mut window_handle) = self.window_handle.take() {
            (window_handle.0).close();
        }
    }

    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        log::info!("Editor open");
        match self.is_open {
            true => false,
            false => {
                // ---------------------------- //
                // 4. Setting up `egui` for use //
                // ---------------------------- //
                self.is_open = true;
                let settings = Settings {
                    window: WindowOpenOptions {
                        title: String::from("synthy"),
                        size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
                        scale: WindowScalePolicy::SystemScaleFactor,
                    },
                    render_settings: RenderSettings::default(),
                };

                let window_handle = EguiWindow::open_parented(
                    &VstParent(parent),
                    settings,
                    self.params.clone(),
                    |_egui_ctx, _queue, _state| {},
                    |egui_ctx: &CtxRef, _, state: &mut Arc<Parameters>| {
                        draw_ui(egui_ctx, state);
                    },
                );

                self.window_handle = Some(WindowParent(window_handle));
                true
            }
        }
    }    
}

// ---------------------------- //
// 4. Wrapper types boilerplate //
// ---------------------------- //
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

struct VstParent(*mut ::std::ffi::c_void);
unsafe impl Send for VstParent {}

pub struct WindowParent(pub WindowHandle);
unsafe impl Send for WindowParent {}
```

Most of this code is boilerplate, so let's cover the important parts only:

### 1. Setting UI size

### 2. Creating `PluginEditor` struct

### 3. Implementing `Editor`

### 4. Setting up `egui` for use

### 5. Wrapper types boilerplate

Because our `Plugin` must implement `Send`, we wrap a few needed pointers in newtypes. We then implement `Send` manually on the newtypes. 

Right below that block, add the following boilerplate code for platform specific windowing:

{{ filename(name="src/editor.rs") }}
```rs
// ...
#[cfg(target_os = "macos")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::macos::MacOSHandle;

        RawWindowHandle::MacOS(MacOSHandle {
            ns_view: self.0 as *mut ::std::ffi::c_void,
            ..MacOSHandle::empty()
        })
    }
}

#[cfg(target_os = "windows")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::windows::WindowsHandle;

        RawWindowHandle::Windows(WindowsHandle {
            hwnd: self.0,
            ..WindowsHandle::empty()
        })
    }
}

#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::unix::XcbHandle;

        RawWindowHandle::Xcb(XcbHandle {
            window: self.0 as u32,
            ..XcbHandle::empty()
        })
    }
}
```

### Adjusting our `lib.rs` to support the editor

We still need to make a few more changes in our `lib.rs` file to enable showing the editor. Add the following field on our `Synthy` struct to keep track of our editor:

```rs
struct Synthy {
    audio: Box<dyn AudioUnit64 + Send>,
    sample_rate: f32,
    parameters: Arc<Parameters>,
    time: Duration,
    note: Option<(Note, Velocity)>,
    enabled: bool,
    // New field
    editor: Option<editor::PluginEditor>,
}
```

Modify the `Plugin::new` method implemented on `Synthy` to return the following struct:

```rs
// Plugin::new
// ...
let params: Arc<Parameters> = Arc::new(Default::default());
Self {
    audio: Box::new(audio_graph) as Box<dyn AudioUnit64 + Send>,
    parameters: params.clone(),
    note: None,
    time: Duration::default(),
    sample_rate: 41_000f32,
    enabled: false,
    editor: Some(editor::PluginEditor {
        params,
        window_handle: None,
        is_open: false,
    }),
}
// ...
```

Lastly, implement the `Plugin::get_editor` method on `Synthy`:

```rs
// Plugin
// ...
fn get_editor(&mut self) -> Option<Box<dyn vst::editor::Editor>> {
    if let Some(editor) = self.editor.take() {
        Some(Box::new(editor) as Box<dyn vst::editor::Editor>)
    } else {
        None
    }
}
```

We can finally create the `draw_ui` function and begin creating a UI, and get our code to compile.

## Using `egui` to create a user interface

The hard, boring part is over! Time to create some cool UIs. Create a new function in your `editor.rs` file titled `draw_ui` with the following parameters. Remember that we called this `draw_ui` function earlier in our `Editor::open` method.

```rs
#[inline(always)]
fn draw_ui(ctx: &CtxRef, params: &mut Arc<Parameters>) -> egui::Response {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.label("hello rust");
            ui.label(format!(
                "Modulation: {}",
                params.get_parameter(crate::Parameter::Modulation as i32)
            ));
        })
    })
    .response
}
```

We use a `CentralPanel` as a base canvas for all our UI content. We then want to show two text labels, so we use `vertical` to position a few `label`s. In our second label, we get the current value of `Parameter::Modulation` and display it. If we want to modify our UI, we will revisit the `draw_ui` function. Almost everything else we've done so far is boilerplate that won't change much. 

### `egui` built-in widgets

### Setting parameters

---

## Footnotes

[^program-files]: Keep in mind that many Windows users store plugins in `C:\Program Files (x86)\VstPlugins` which has strict permissions. In my testing, if the folder specified was incorrect, my host failed to even _list_ the plugin as available.

[^credits]: It is important to mention the majority of the initialization is adapted from [egui_baseview_test_vst2](https://github.com/DGriffin91/egui_baseview_test_vst2/blob/main/src/lib.rs) by [DGriffin91](https://github.com/DGriffin91), licensed under MIT.