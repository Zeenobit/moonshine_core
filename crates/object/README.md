# 🌴 Moonshine Object

[![crates.io](https://img.shields.io/crates/v/moonshine-object)](https://crates.io/crates/moonshine-object)
[![downloads](https://img.shields.io/crates/dr/moonshine-object?label=downloads)](https://crates.io/crates/moonshine-object)
[![docs.rs](https://docs.rs/moonshine-object/badge.svg)](https://docs.rs/moonshine-object)
[![license](https://img.shields.io/crates/l/moonshine-object)](https://github.com/Zeenobit/moonshine_object/blob/main/LICENSE)
[![stars](https://img.shields.io/github/stars/Zeenobit/moonshine_object)](https://github.com/Zeenobit/moonshine_object)

An extension to [Bevy](https://bevyengine.org) which provides an ergonomic interface for managing complex [`Entity`] hierarchies.

Entities are nice. Objects are *better*! 😎

## Overview

This crate is designed to provide a wrapper for some commonly used operations when working with entities in Bevy.

It is often required for various systems to be able to traverse complex entity hierarchies. This is especially true for initialization code when various components need to reference various entities within a hierarchy of entities.

For example, consider a system which reacts to a flying bird by flapping its wings:

```rust
use bevy::prelude::*;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Flying;

fn setup_bird(
    query: Query<Entity, (With<Bird>, Added<Flying>)>,
    children_query: Query<&Children>,
    name: Query<&Name>,
    mut commands: Commands
) {
    for entity in query.iter() {
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                if let Ok(name) = name.get(*child) {
                    if name.as_str() == "Wings" {
                        if let Ok(wings) = children_query.get(*child) {
                            for wing in wings.iter() {
                                // TODO: Flap! Flap!
                            }
                        }
                    }
                }
            }
        }
    }
}
```

This code is intentionally verbose to show the hierarchy complexity.

This crate tries to make these situations more ergonomic by introducing [`Object<T>`].

It behaves like an [`Entity`] or [`Instance<T>`] with some extra features:

```rust
use bevy::prelude::*;
use moonshine_object::prelude::*;

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

### Features

- Less boilerplate when dealing with complex entity hierarchies
- Full type safety enforced through [`Kind`] semantics
- No macros! No registration!

## Usage

### `Objects<T>`

Use [`Objects<T>`] as a system parameter to access all [`Object<T>`] instances.

This [`SystemParam`] is designed to be used like a [`Query`]:

```rust
use bevy::prelude::*;
use moonshine_object::prelude::*;

#[derive(Component)]
struct Bird;

fn update_birds(birds: Objects<Bird>) {
    for bird in birds.iter() {
        // ...
    }
}
```

Like a [`Query`], you may also use a [`QueryFilter`]:

```rust
use bevy::prelude::*;
use moonshine_object::prelude::*;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Flying;

fn update_flying_birds(birds: Objects<Bird, With<Flying>>) {
    for bird in birds.iter() {
        // ...
    }
}
```

Internally, [`Objects<T>`] is just a thin wrapper around some common queries:

- `Query<Instance<T>>`
- `Query<&Parent>` / `Query<&Children>`
- `Query<&Name>`

### `Object<T>`

Each [`Object<T>`] is a reference to an [`Entity`] with type, name, and hierarchy information. This provides a convenient way to pass this data between functions:

```rust
use bevy::prelude::*;
use moonshine_object::prelude::*;

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Flying;

fn update_flying_birds(birds: Objects<Bird, With<Flying>>) {
    for bird in birds.iter() {
        flap_wings(bird);
    }
}

fn flap_wings(bird: Object<Bird>) {
    if let Some(wings) = bird.find_by_path("./Wings") {
        for wing in wings.children() {
            // TODO: Flap! Flap!
        }
    }
}
```

⚠️ Unlike an [`Entity`] or [`Instance<T>`], [`Object<T>`] has a non-static lifetime and may not be used as a [`Query`] term.

### Casting

Like [`Instance<T>`], any [`Object<T>`] may be be cast into an [`Object<U>`][`Object`] if `T` implements [`CastInto<U>`](https://docs.rs/moonshine-kind/latest/moonshine_kind/trait.CastInto.html).

You may implement this trait for your own kinds using the [`safe_cast`](https://docs.rs/moonshine-kind/latest/moonshine_kind/macro.safe_cast.html) macro:

```rust
use bevy::prelude::*;
use moonshine_kind::prelude::*;
use moonshine_object::prelude::*;

#[derive(Component)]
struct Bird;

struct Creature;

// Every Bird is a Creature by definition:
impl Kind for Creature {
    type Filter = (With<Bird>, /* ... */);
}

// Therefore, all birds may safely be cast into creatures:
safe_cast!(Bird => Creature);

// Birds can chirp.
fn chirp(bird: Object<Bird>) {
    // TODO: Chirp!
}

// Creatures can find food.
fn find_food(creature: Object<Creature>) {
    // TODO: Find food!
}

// Birds chirp when they get hungry.
fn handle_hunger(bird: Object<Bird>) {
    chirp(bird);
    find_food(bird.cast_into()); // Safe! :)
}

```

Any [`Object<T>`] is safely convertible to [`Object<Any>`][`Object`].

## Support

Please [post an issue](https://github.com/Zeenobit/moonshine_object/issues/new) for any bugs, questions, or suggestions.

You may also contact me on the official [Bevy Discord](https://discord.gg/bevy) server as **@Zeenobit**.

[`Entity`]:https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`Component`]:https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html
[`Query`]:https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html
[`SystemParam`]:https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html
[`QueryFilter`]:https://docs.rs/bevy/latest/bevy/ecs/query/trait.QueryFilter.html
[`Kind`]:https://docs.rs/moonshine-kind/0.1.4/moonshine_kind/trait.Kind.html
[`Instance<T>`]:https://docs.rs/moonshine-kind/latest/moonshine_kind/struct.Instance.html
[`Objects<T>`]:https://docs.rs/moonshine-object/latest/moonshine_object/struct.Objects.html
[`Object<T>`]:https://docs.rs/moonshine-object/latest/moonshine_object/struct.Object.html
[`Object`]:https://docs.rs/moonshine-object/latest/moonshine_object/struct.Object.html