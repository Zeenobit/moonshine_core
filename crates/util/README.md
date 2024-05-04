# 🛠️ Moonshine Utilities

[![crates.io](https://img.shields.io/crates/v/moonshine-util)](https://crates.io/crates/moonshine-util)
[![downloads](https://img.shields.io/crates/dr/moonshine-util?label=downloads)](https://crates.io/crates/moonshine-util)
[![docs.rs](https://docs.rs/moonshine-util/badge.svg)](https://docs.rs/moonshine-util)
[![license](https://img.shields.io/crates/l/moonshine-util)](https://github.com/Zeenobit/moonshine_util/blob/main/LICENSE)
[![stars](https://img.shields.io/github/stars/Zeenobit/moonshine_util)](https://github.com/Zeenobit/moonshine_util)

Collection of utilities for [Bevy](https://github.com/bevyengine/bevy).

## Features

### `Expect<T>`

A decorator for [`QueryData`](https://docs.rs/bevy/latest/bevy/ecs/query/trait.QueryData.html) which panics if it doesn't match.

This helps avoid silent failures in systems due to missing components:

```rust
use bevy::prelude::*;
use moonshine_util::expect::Expect;

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

#[derive(Bundle)]
struct AB {
    a: A, // Every `A` is expected to have a `B`
    b: B,
}

fn bad_system(mut commands: Commands) {
    commands.spawn(A); // BUG: Spawn A witout B!
}

fn unsafe_system(mut query: Query<(&A, &B)>) {
    for (a, b) in query.iter() {
        // An instance of `A` does exist, but this system skips over it silently!
    }
}

fn safe_system(mut query: Query<(&A, Expect<&B>)>) {
    for (a, b) in query.iter() {
        // This system will panic if an `A` instance is missing a `B`!
    }
}
```

### `HierarchyQuery`

A convenient [`SystemParam`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html) for traversing and querying entity hierarchies:

```rust
use bevy::prelude::*;
use moonshine_util::hierarchy::HierarchyQuery;

#[derive(Component)]
struct Needle;

#[derive(Component)]
struct Haystack;

fn spawn_haystack(mut commands: Commands) {
    // A complex hierarchy ...
    commands.spawn(Haystack).with_children(|x| {
        x.spawn_empty().with_children(|y| {
            y.spawn_empty().with_children(|z| {
                z.spawn(Needle);
            });
        });
    });
}

fn find_needle(
    haystack: Query<Entity, With<Haystack>>,
    needle_query: Query<Entity, With<Needle>>,
    hierarchy: HierarchyQuery
) {
    let haystack = haystack.single();
    if let Some(needle) = hierarchy.find_descendant(haystack, &needle_query) {
        // ...
    }
}
```

Some useful functions include:

- `fn parent(&self, Entity) -> Option<Entity>`
- `fn has_parent(&self, Entity) -> bool`
- `fn children(&self, Entity) -> Iterator<Item = Entity>`
- `fn has_children(&self, Entity) -> bool`
- `fn root(&self, Entity) -> Entity`
- `fn is_root(&self, Entity) -> bool`
- `fn ancestors(&self, Entity) -> Iterator<Item = Entity>`
- `fn descendants(&self, Entity) -> Iterator<Item = Entity>`
- `fn is_ancestor_of(&self, Entity, Entity) -> bool`
- `fn is_descendant_of(&self, Entity, Entity) -> bool`
- `fn find_ancestor<T, F>(&self, Entity, &Query<T, F>) -> Option<QueryItem<T>>`
- `fn find_descendant<T, F>(&self, Entity, &Query<T, F>) -> Option<QueryItem<T>>`

See code documentation for complete details.

### `RunSystemLoop`

A trait similar to [`RunSystemOnce`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.RunSystemOnce.html) which allows you to run a system loop for testing purposes:

```rust
use bevy::prelude::*;
use moonshine_util::diagnostics::RunSystemLoop;

let mut world = World::new();
let outputs = world.run_system_loop(2, |mut commands: Commands| {
    commands.spawn_empty().id()
});

assert_eq!(outputs.len(), 2);

assert!(world.get_entity(outputs[0]).is_some());
assert!(world.get_entity(outputs[1]).is_some());
```

### Utility Systems

A collection of simple and generic systems useful for constructing larger system pipelines:

- `has_event<T: Event>() -> bool`
- `has_resource<T: Resource>() -> bool`
- `remove_resource<T: Resource>(Commands)`
- `remove_resource_immediate<T: Resource>(&mut World)`

See code documentation for usage examples.

## Support

Please [post an issue](https://github.com/Zeenobit/moonshine_util/issues/new) for any bugs, questions, or suggestions.

You may also contact me on the official [Bevy Discord](https://discord.gg/bevy) server as **@Zeenobit**.