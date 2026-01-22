# 🍸 Moonshine Core

Unconventional cocktail of libraries to make ECS-driven development easier and safer in [Bevy](https://bevyengine.org/).

See individual crates for detailed documentation and examples.

### 🍎 Moonshine Kind

[![crates.io](https://img.shields.io/crates/v/moonshine-kind)](https://crates.io/crates/moonshine-kind)
[![downloads](https://img.shields.io/crates/dr/moonshine-kind?label=downloads)](https://crates.io/crates/moonshine-kind)
[![docs.rs](https://docs.rs/moonshine-kind/badge.svg)](https://docs.rs/moonshine-kind)
[![license](https://img.shields.io/crates/l/moonshine-kind)](https://github.com/Zeenobit/moonshine_kind/blob/main/LICENSE)
[![stars](https://img.shields.io/github/stars/Zeenobit/moonshine_kind)](https://github.com/Zeenobit/moonshine_kind)

Type safety solution for Bevy entities:

```rust
use bevy::prelude::*;
use moonshine_core::prelude::*;

#[derive(Component)]
struct Fruit;

#[derive(Component)]
struct FruitBasket {
    fruits: Vec<Instance<Fruit>>
}
```

### 🌴 Moonshine Object

[![crates.io](https://img.shields.io/crates/v/moonshine-object)](https://crates.io/crates/moonshine-object)
[![downloads](https://img.shields.io/crates/dr/moonshine-object?label=downloads)](https://crates.io/crates/moonshine-object)
[![docs.rs](https://docs.rs/moonshine-object/badge.svg)](https://docs.rs/moonshine-object)
[![license](https://img.shields.io/crates/l/moonshine-object)](https://github.com/Zeenobit/moonshine_object/blob/main/LICENSE)
[![stars](https://img.shields.io/github/stars/Zeenobit/moonshine_object)](https://github.com/Zeenobit/moonshine_object)

Ergonomic wrapper for managing complex enttiy hierarchies:

```rust
use bevy::prelude::*;
use moonshine_core::prelude::*;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Flying;

fn setup_bird(birds: Objects<Bird, Added<Flying>>, mut commands: Commands) {
    for bird in birds.iter() {
        if let Some(wings) = bird.find_by_path("./Wings") {
            for wing in wings.children() {
                // TODO: Flap! Flap!
            }
        }
    }
}
```

### 💾 Moonshine Save

[![crates.io](https://img.shields.io/crates/v/moonshine-save)](https://crates.io/crates/moonshine-save)
[![downloads](https://img.shields.io/crates/dr/moonshine-save?label=downloads)](https://crates.io/crates/moonshine-save)
[![docs.rs](https://docs.rs/moonshine-save/badge.svg)](https://docs.rs/moonshine-save)
[![license](https://img.shields.io/crates/l/moonshine-save)](https://github.com/Zeenobit/moonshine_save/blob/main/LICENSE)
[![stars](https://img.shields.io/github/stars/Zeenobit/moonshine_save)](https://github.com/Zeenobit/moonshine_save)

Save/Load framework for managing persistent game state:

```rust
use bevy::prelude::*;
use moonshine_core::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Save)]
struct Player { /* ... */ }

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .register_type::<Player>()
        .add_observer(save_on_default_event)
        .add_observer(load_on_default_event)
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (trigger_save, trigger_load)
        );

    // app.run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player { /* ... */ });
}

fn trigger_save(key: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if key.just_pressed(KeyCode::KeyS) {
        commands.trigger_save(SaveWorld::default_into_file("world.ron"));
    }
}

fn trigger_load(key: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if key.just_pressed(KeyCode::KeyL) {
        commands.trigger_load(LoadWorld::default_from_file("world.ron"));
    }
}
```

### 🏷️ Moonshine Tag

[![crates.io](https://img.shields.io/crates/v/moonshine-tag)](https://crates.io/crates/moonshine-tag)
[![downloads](https://img.shields.io/crates/dr/moonshine-tag?label=downloads)](https://crates.io/crates/moonshine-tag)
[![docs.rs](https://docs.rs/moonshine-tag/badge.svg)](https://docs.rs/moonshine-tag)
[![license](https://img.shields.io/crates/l/moonshine-tag)](https://github.com/Zeenobit/moonshine_tag/blob/main/LICENSE)
[![stars](https://img.shields.io/github/stars/Zeenobit/moonshine_tag)](https://github.com/Zeenobit/moonshine_tag)

Cheap, fast, mostly unique identifiers designed for Bevy:

```rust
use bevy::prelude::*;
use moonshine_tag::prelude::*;

tags! { APPLE, ORANGE, JUICY, CRUNCHY, POISONED }

let mut world = World::new();

// Define some fruits!
let fruits = [
    Tags::from([APPLE, CRUNCHY]),
    Tags::from([ORANGE, JUICY]),
    Tags::from([APPLE, CRUNCHY, POISONED])
];

// Only crunchy, edible apples, please! :)
let filter: TagFilter = tag_filter!([APPLE, CRUNCHY] & ![POISONED]);

for fruit in &fruits {
    if filter.allows(fruit) {
        world.spawn(fruit.clone());
    }
}

assert!(fruits[0].matches(&filter));
assert!(!fruits[1].matches(&filter));
assert!(!fruits[2].matches(&filter));
```


### 🛠️ Moonshine Utilities

Collection of generic utilities for improved safety, diagnostics, and ergonomics.

[![crates.io](https://img.shields.io/crates/v/moonshine-util)](https://crates.io/crates/moonshine-util)
[![downloads](https://img.shields.io/crates/dr/moonshine-util?label=downloads)](https://crates.io/crates/moonshine-util)
[![docs.rs](https://docs.rs/moonshine-util/badge.svg)](https://docs.rs/moonshine-util)
[![license](https://img.shields.io/crates/l/moonshine-util)](https://github.com/Zeenobit/moonshine_util/blob/main/LICENSE)
[![stars](https://img.shields.io/github/stars/Zeenobit/moonshine_util)](https://github.com/Zeenobit/moonshine_util)

## Support

Please [post an issue](https://github.com/Zeenobit/moonshine_core/issues/new) for any bugs, questions, or suggestions.

You may also contact me on the official [Bevy Discord](https://discord.gg/bevy) server as **@Zeenobit**.
