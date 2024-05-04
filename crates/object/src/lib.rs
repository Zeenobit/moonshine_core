#![doc = include_str!("../README.md")]

use std::fmt;

use bevy_core::Name;
use bevy_ecs::{
    prelude::*,
    query::{QueryData, QueryEntityError, QueryFilter, QueryItem},
    system::SystemParam,
};
use moonshine_kind::prelude::*;
use moonshine_util::hierarchy::HierarchyQuery;

pub mod prelude {
    pub use super::{Object, Objects};
}

pub use moonshine_kind::{Any, CastInto, Kind};

/// A [`SystemParam`] similar to [`Query`] which provides [`Object<T>`] access for its items.
#[derive(SystemParam)]
pub struct Objects<'w, 's, T = Any, F = ()>
where
    T: Kind,
    F: 'static + QueryFilter,
{
    pub instance: Query<'w, 's, Instance<T>, F>,
    pub hierarchy: HierarchyQuery<'w, 's>,
    pub name: Query<'w, 's, &'static Name>,
}

impl<'w, 's, T, F> Objects<'w, 's, T, F>
where
    T: Kind,
    F: 'static + QueryFilter,
{
    /// Iterates over all [`Object`]s of [`Kind`] `T` which match the [`QueryFilter`] `F`.
    pub fn iter(&self) -> impl Iterator<Item = Object<'w, 's, '_, T>> {
        self.instance.iter().map(|instance| Object {
            instance,
            hierarchy: &self.hierarchy,
            name: &self.name,
        })
    }

    /// Gets the [`Object`] of [`Kind`] `T` from an [`Entity`], if it matches.
    pub fn get(&self, entity: Entity) -> Result<Object<'w, 's, '_, T>, QueryEntityError> {
        self.instance.get(entity).map(|instance| Object {
            instance,
            hierarchy: &self.hierarchy,
            name: &self.name,
        })
    }

    /// Gets the [`Object`] of [`Kind`] `T` from an [`Instance`].
    ///
    /// # Safety
    /// Assumes `instance` is a valid [`Instance`] of [`Kind`] `T`.
    pub fn instance(&self, instance: Instance<T>) -> Object<'w, 's, '_, T> {
        self.get(instance.entity()).expect("instance must be valid")
    }
}

/// Represents an [`Entity`] of [`Kind`] `T` with hierarchy and name information.
pub struct Object<'w, 's, 'a, T: Kind = Any> {
    instance: Instance<T>,
    hierarchy: &'a HierarchyQuery<'w, 's>,
    name: &'a Query<'w, 's, &'static Name>,
}

impl<'w, 's, 'a, T: Kind> Object<'w, 's, 'a, T> {
    /// Creates a new [`Object<T>`] from an [`Object<Any>`].
    ///
    /// This is semantically equivalent to an unsafe downcast.
    ///
    /// # Safety
    /// Assumes `base` is of [`Kind`] `T`.
    pub unsafe fn from_base_unchecked(base: Object<'w, 's, 'a>) -> Self {
        Self {
            instance: base.instance.cast_into_unchecked(),
            hierarchy: base.hierarchy,
            name: base.name,
        }
    }

    /// Returns this object as an [`Instance<T>`].
    pub fn instance(&self) -> Instance<T> {
        self.instance
    }

    /// Returns this object as an [`Entity`].
    pub fn entity(&self) -> Entity {
        self.instance.entity()
    }

    /// Returns the [`Name`] of this object.
    pub fn name(&self) -> Option<&str> {
        self.name.get(self.entity()).ok().map(|name| name.as_str())
    }

    /// Returns the [`Name`] of this object if it has one, or a given default string otherwise.
    pub fn name_or_default(&self, default: &'a str) -> &'a str {
        self.name
            .get(self.entity())
            .map(|name| name.as_str())
            .unwrap_or(default)
    }

    /// Returns true if this object has no parent.
    pub fn is_root(&self) -> bool {
        self.hierarchy.is_root(self.entity())
    }

    #[deprecated(note = "use `has_children` instead")]
    pub fn is_parent(&self) -> bool {
        self.has_children()
    }

    /// Returns true if this object has a parent.
    pub fn is_child(&self) -> bool {
        self.parent().is_some()
    }

    /// Returns true if this object is a child of the given `parent` [`Entity`].
    pub fn is_child_of(&self, parent: Entity) -> bool {
        self.hierarchy.is_child_of(self.entity(), parent)
    }

    /// Returns true if this object has some children.
    pub fn has_children(&self) -> bool {
        self.hierarchy.has_children(self.entity())
    }

    /// Returns true if this object is a descendant of the given `ancestor` [`Entity`].
    pub fn is_descendant_of(&self, ancestor: Entity) -> bool {
        self.hierarchy.is_descendant_of(self.entity(), ancestor)
    }

    /// Attempts to find an object by its path, relative to this one.
    ///
    /// # Usage
    ///
    /// An **Object Path** is a string of object names separated by slashes which represents
    /// the path to an object within a hierarchy.
    ///
    /// In additional to object names, the path may contain the following special characters:
    ///   - `.` represents this object.
    ///   - `..` represents the parent object.
    ///   - `*` represents any child object.
    ///
    /// Note that this method of object search is relatively slow, and should be reserved for
    /// when performance is not the top priority, such as during initialization or prototyping.
    ///
    /// Instead, prefer to use [`Component`] to tag your entities and [`Query`] them instead, if possible.
    ///
    /// # Safety
    /// This method is somewhat experimental with plans for future expansion.
    /// Please [report](https://github.com/Zeenobit/moonshine_object/issues) any bugs you encounter or features you'd like.
    pub fn find_by_path(&self, path: impl AsRef<str>) -> Option<Object<'w, 's, 'a>> {
        let tail: Vec<&str> = path.as_ref().split('/').collect();
        find_by_path(self.cast_into(), &tail)
    }

    /// Returns the root of this object's hierarchy.
    pub fn root(&self) -> Object<'w, 's, 'a> {
        self.rebind_any(self.hierarchy.root(self.entity()))
    }

    /// Returns the parent of this object.
    pub fn parent(&self) -> Option<Object<'w, 's, 'a>> {
        self.hierarchy
            .parent(self.entity())
            .map(|entity| self.rebind_any(entity))
    }

    /// Iterates over all children of this object.
    pub fn children(&self) -> impl Iterator<Item = Object<'w, 's, 'a>> + '_ {
        self.hierarchy
            .children(self.entity())
            .map(|entity| self.rebind_any(entity))
    }

    /// Iterates over this object in addition to all its ancestors.
    pub fn self_and_ancestors(&self) -> impl Iterator<Item = Object<'w, 's, 'a>> + '_ {
        std::iter::once(self.cast_into()).chain(self.ancestors())
    }

    /// Iterates over all ancestors of this object.
    pub fn ancestors(&self) -> impl Iterator<Item = Object<'w, 's, 'a>> + '_ {
        self.hierarchy
            .ancestors(self.entity())
            .map(|entity| self.rebind_any(entity))
    }

    /// Queries all ancestors of this object with a given [`Query`].
    pub fn query_ancestors<Q: QueryData>(
        &'a self,
        query: &'a Query<'w, 's, Q>,
    ) -> impl Iterator<Item = QueryItem<'_, Q::ReadOnly>> + 'a {
        self.ancestors().filter_map(move |object| {
            let entity = object.entity();
            query.get(entity).ok()
        })
    }

    /// Iterates over this object in addition to all its descendants.
    pub fn self_and_descendants(&self) -> impl Iterator<Item = Object<'w, 's, 'a>> + '_ {
        std::iter::once(self.cast_into()).chain(self.descendants())
    }

    /// Iterates over all descendants of this object.
    pub fn descendants(&self) -> impl Iterator<Item = Object<'w, 's, 'a>> + '_ {
        self.hierarchy
            .descendants(self.entity())
            .map(|entity| self.rebind_any(entity))
    }

    /// Queries all descendants of this object with a given [`Query`].
    pub fn query_descendants<Q: QueryData>(
        &'a self,
        query: &'a Query<'w, 's, Q>,
    ) -> impl Iterator<Item = QueryItem<'_, Q::ReadOnly>> + 'a {
        self.descendants().filter_map(move |object| {
            let entity = object.entity();
            query.get(entity).ok()
        })
    }

    /// Uses this object to create a new [`Object`] which references the given `instance` of the same [`Kind`].
    ///
    /// This function is useful when you already have an [`Object<T>`] and another [`Instance<T>`].
    /// It gives you type-safe object access to the other instance.
    pub fn rebind(&self, instance: Instance<T>) -> Object<'w, 's, 'a, T> {
        Object {
            instance,
            hierarchy: self.hierarchy,
            name: self.name,
        }
    }

    /// Uses this object to create a new [`Object`] which references the given [`Entity`].
    ///
    /// This function is useful when you already have an [`Object<T>`] and another [`Entity`].
    /// It gives you generic object access to the other entity.
    pub fn rebind_any(&self, entity: Entity) -> Object<'w, 's, 'a> {
        Object {
            instance: Instance::from(entity),
            hierarchy: self.hierarchy,
            name: self.name,
        }
    }

    /// Uses this object to create a new [`Object`] which references the given `instance` of a different [`Kind`].
    ///
    /// This function is useful when you already have an [`Object<T>`] and another [`Instance<U>`].
    /// It gives you type-safe object access to the other instance.
    ///
    /// Note that this function assumes the given instance is a valid instance of the given kind.
    pub fn rebind_as<U: Kind>(&self, instance: Instance<U>) -> Object<'w, 's, 'a, U> {
        Object {
            instance,
            hierarchy: self.hierarchy,
            name: self.name,
        }
    }

    /// Safety casts this object into another [`Kind`].
    ///
    /// See [`CastInto`] for more information.
    pub fn cast_into<U: Kind>(self) -> Object<'w, 's, 'a, U>
    where
        T: CastInto<U>,
    {
        Object {
            instance: self.instance.cast_into(),
            hierarchy: self.hierarchy,
            name: self.name,
        }
    }

    /// Returns this object as an [`Object<Any>`].
    pub fn as_any(&self) -> Object<'_, '_, '_> {
        self.cast_into()
    }

    /// Casts this object into another [`Kind`] without any safety checks.
    ///
    /// This is semantically equivalent to a raw C-style cast.
    ///
    /// # Safety
    /// Assumes any instance of kind `T` is also a valid instance of kind `U`.
    pub unsafe fn cast_into_unchecked<U: Kind>(self) -> Object<'w, 's, 'a, U> {
        Object {
            instance: self.instance.cast_into_unchecked(),
            hierarchy: self.hierarchy,
            name: self.name,
        }
    }
}

impl<'w, 's, 'a, T: Component> Object<'w, 's, 'a, T> {
    pub fn from_base(world: &World, object: Object<'w, 's, 'a>) -> Option<Object<'w, 's, 'a, T>> {
        let entity = world.entity(object.entity());
        let instance = Instance::<T>::from_entity(entity)?;
        // SAFE: Entity was just checked to a valid instance of T.
        Some(object.rebind_as(instance))
    }
}

impl<T: Kind> Clone for Object<'_, '_, '_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Kind> Copy for Object<'_, '_, '_, T> {}

impl<T: Kind> From<Object<'_, '_, '_, T>> for Entity {
    fn from(object: Object<'_, '_, '_, T>) -> Self {
        object.entity()
    }
}

impl<T: Kind> From<Object<'_, '_, '_, T>> for Instance<T> {
    fn from(object: Object<'_, '_, '_, T>) -> Self {
        object.instance()
    }
}

impl<T: Kind> PartialEq for Object<'_, '_, '_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.instance == other.instance
    }
}

impl<T: Kind> Eq for Object<'_, '_, '_, T> {}

impl<T: Kind> fmt::Debug for Object<'_, '_, '_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_tuple(&T::debug_name());
        out.field(&self.entity());
        if let Some(name) = self.name() {
            out.field(&name);
        }
        out.finish()
    }
}

fn find_by_path<'w, 's, 'a>(curr: Object<'w, 's, 'a>, tail: &[&str]) -> Option<Object<'w, 's, 'a>> {
    if tail.is_empty() {
        return Some(curr);
    }

    let head = tail[0];
    let tail = &tail[1..];

    if head == "." || head.is_empty() {
        find_by_path(curr, tail)
    } else if head == ".." {
        if let Some(parent) = curr
            .hierarchy
            .parent(curr.entity())
            .map(|parent| curr.rebind_any(parent))
        {
            find_by_path(parent, tail)
        } else {
            None
        }
    } else if head == "*" {
        for child in curr.hierarchy.children(curr.entity()) {
            let child = curr.rebind_any(child);
            if let Some(result) = find_by_path(child, tail) {
                return Some(result);
            }
        }
        return None;
    } else if let Some(child) = curr
        .hierarchy
        .children(curr.entity())
        .map(|child| curr.rebind_any(child))
        .find(|part| part.name().is_some_and(|name| name == head))
    {
        find_by_path(child, tail)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bevy::{ecs::system::RunSystemOnce, prelude::*};

    #[test]
    fn find_by_path() {
        let mut w = World::new();

        //     A
        //    /
        //   B
        //  / \
        // C   D

        let (a, b, c, d) = w.run_system_once(|mut commands: Commands| {
            let a = commands.spawn(Name::new("A")).id();
            let b = commands.spawn(Name::new("B")).id();
            let c = commands.spawn(Name::new("C")).id();
            let d = commands.spawn(Name::new("D")).id();

            commands.entity(a).push_children(&[b]);
            commands.entity(b).push_children(&[c, d]);

            (a, b, c, d)
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects.get(a).unwrap().find_by_path("").unwrap().entity();
            assert_eq!(a, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects.get(a).unwrap().find_by_path(".").unwrap().entity();
            assert_eq!(a, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects.get(a).unwrap().find_by_path("B").unwrap().entity();
            assert_eq!(b, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects
                .get(a)
                .unwrap()
                .find_by_path("B/C")
                .unwrap()
                .entity();
            assert_eq!(c, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects
                .get(a)
                .unwrap()
                .find_by_path("B/D")
                .unwrap()
                .entity();
            assert_eq!(d, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects
                .get(a)
                .unwrap()
                .find_by_path("B/*")
                .unwrap()
                .entity();
            assert_eq!(c, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects
                .get(a)
                .unwrap()
                .find_by_path("*/D")
                .unwrap()
                .entity();
            assert_eq!(d, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects
                .get(a)
                .unwrap()
                .find_by_path("*/*")
                .unwrap()
                .entity();
            assert_eq!(c, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects.get(b).unwrap().find_by_path("..").unwrap().entity();
            assert_eq!(a, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects.get(c).unwrap().find_by_path("..").unwrap().entity();
            assert_eq!(b, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects
                .get(c)
                .unwrap()
                .find_by_path("../D")
                .unwrap()
                .entity();
            assert_eq!(d, x);
        });

        w.run_system_once(move |objects: Objects| {
            let x = objects
                .get(c)
                .unwrap()
                .find_by_path("../C")
                .unwrap()
                .entity();
            assert_eq!(c, x);
        });
    }
}
