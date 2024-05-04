use bevy_ecs::{
    prelude::*,
    query::{QueryData, QueryFilter, QueryItem, ReadOnlyQueryData},
    system::SystemParam,
};
use bevy_hierarchy::prelude::*;

/// A [`SystemParam`] for ergonomic [`Entity`] hierarchy traversal.
///
/// # Example
/// ```
/// use bevy::prelude::*;
/// use moonshine_util::hierarchy::HierarchyQuery;
///
/// #[derive(Component)]
/// struct Needle;
///
/// #[derive(Component)]
/// struct Haystack;
///
/// fn spawn_haystack(mut commands: Commands) {
///     commands.spawn(Haystack).with_children(|x| {
///         x.spawn_empty().with_children(|y| {
///             y.spawn_empty().with_children(|z| {
///                 z.spawn(Needle);
///             });
///         });
///     });
/// }
///
/// fn find_needle(
///     haystack: Query<Entity, With<Haystack>>,
///     needle: Query<Entity, With<Needle>>,
///     hierarchy: HierarchyQuery
/// ) {
///     let haystack = haystack.single();
///     let needle = hierarchy.find_descendant(haystack, &needle);
///     assert!(needle.is_some());
/// }
///
/// # bevy_ecs::system::assert_is_system(spawn_haystack);
/// # bevy_ecs::system::assert_is_system(find_needle);
/// ```
#[derive(SystemParam)]
pub struct HierarchyQuery<'w, 's> {
    parent: Query<'w, 's, &'static Parent>,
    children: Query<'w, 's, &'static Children>,
}

impl<'w, 's> HierarchyQuery<'w, 's> {
    /// Returns the parent of the given entity, if it has one.
    pub fn parent(&self, entity: Entity) -> Option<Entity> {
        self.parent.get(entity).ok().map(|parent| **parent)
    }

    /// Returns true if the given entity has a parent.
    pub fn has_parent(&self, entity: Entity) -> bool {
        self.parent(entity).is_some()
    }

    /// Iterates over the children of the given entity.
    pub fn children(&self, entity: Entity) -> impl Iterator<Item = Entity> + '_ {
        self.children
            .get(entity)
            .ok()
            .into_iter()
            .flat_map(|children| children.into_iter().copied())
    }

    /// Returns true if the given entity has children.
    pub fn has_children(&self, entity: Entity) -> bool {
        self.children
            .get(entity)
            .ok()
            .map(|children| !children.is_empty())
            .unwrap_or(false)
    }

    /// Returns the root of the given entity's hierarchy.
    pub fn root(&self, entity: Entity) -> Entity {
        let mut root = entity;
        while let Some(parent) = self.parent(root) {
            root = parent;
        }
        root
    }

    /// Returns true if the given `entity` is the root of its hierarchy.
    pub fn is_root(&self, entity: Entity) -> bool {
        self.parent(entity).is_none()
    }

    /// Iterates over the ancestors of the given `entity`.
    pub fn ancestors(&self, entity: Entity) -> impl Iterator<Item = Entity> + '_ {
        self.parent.iter_ancestors(entity)
    }

    /// Iterates over the descendants of the given `entity`.
    pub fn descendants(&self, entity: Entity) -> impl Iterator<Item = Entity> + '_ {
        self.children.iter_descendants(entity)
    }

    /// Returns true if given `entity` is an ancestor of the given `descendant`.
    pub fn is_ancestor_of(&self, entity: Entity, descendant: Entity) -> bool {
        self.ancestors(descendant).any(|parent| parent == entity)
    }

    /// Returns true if given `entity` is a child of the given `parent`.
    pub fn is_child_of(&self, entity: Entity, parent: Entity) -> bool {
        self.parent(entity).map(|p| p == parent).unwrap_or(false)
    }

    /// Returns true if given `entity` is a descendant of the given `ancestor`.
    pub fn is_descendant_of(&self, entity: Entity, ancestor: Entity) -> bool {
        self.ancestors(entity).any(|parent| parent == ancestor)
    }

    /// Returns the first ancestor of the given `entity` that matches the given `query`.
    pub fn find_ancestor<'a, T: ReadOnlyQueryData, F: QueryFilter>(
        &self,
        entity: Entity,
        query: &'a Query<T, F>,
    ) -> Option<QueryItem<'a, T::ReadOnly>> {
        self.ancestors(entity)
            .find_map(|ancestor| query.get(ancestor).ok())
    }

    /// Returns the first ancestor of the given `entity` that matches the given mutable `query`.
    pub fn find_ancestor_mut<'a, T: QueryData, F: QueryFilter>(
        &self,
        mut entity: Entity,
        query: &'a mut Query<T, F>,
    ) -> Option<QueryItem<'a, T>> {
        while let Some(parent) = self.parent(entity) {
            if query.get(parent).is_ok() {
                return Some(query.get_mut(parent).unwrap());
            }
            entity = parent;
        }
        None
    }

    /// Returns the first descendant of the given `entity` that matches the given `query`.
    pub fn find_descendant<'a, T: ReadOnlyQueryData, F: QueryFilter>(
        &self,
        entity: Entity,
        query: &'a Query<T, F>,
    ) -> Option<QueryItem<'a, T::ReadOnly>> {
        self.descendants(entity)
            .find_map(|descendant| query.get(descendant).ok())
    }

    // TODO: find_descendant_mut
}

#[cfg(test)]
mod tests {
    use bevy_ecs::system::RunSystemOnce;

    use super::*;

    #[test]
    fn parent_child() {
        let mut w = World::new();
        let e = w
            .spawn_empty()
            .with_children(|parent| {
                parent.spawn_empty();
            })
            .id();

        let pass = w.run_system_once(move |q: HierarchyQuery| {
            assert!(q.is_root(e));
            assert!(q.has_children(e));
            assert_eq!(q.children(e).count(), 1);
            true
        });

        assert!(pass);
    }

    #[test]
    fn query() {
        #[derive(Component)]
        struct A;

        #[derive(Component)]
        struct B;

        #[derive(Component)]
        struct C;

        let mut w = World::new();
        let a = w
            .spawn(A)
            .with_children(|a| {
                a.spawn(B).with_children(|b| {
                    b.spawn(C);
                });
            })
            .id();

        let pass = w.run_system_once(
            move |q: HierarchyQuery,
                  qa: Query<Entity, With<A>>,
                  qb: Query<Entity, With<B>>,
                  qc: Query<Entity, With<C>>| {
                let c_from_a = q.find_descendant(a, &qc).unwrap();
                let b_from_c = q.find_ancestor(c_from_a, &qb).unwrap();
                let b_from_a = q.find_descendant(a, &qb).unwrap();
                let a_from_c = q.find_ancestor(c_from_a, &qa).unwrap();
                let a_from_b = q.find_ancestor(b_from_c, &qa).unwrap();
                let c_from_b = q.find_descendant(b_from_a, &qc).unwrap();
                assert_eq!(c_from_a, c_from_b);
                assert_eq!(b_from_a, b_from_c);
                assert_eq!(a_from_c, a_from_b);
                assert_eq!(a_from_c, a);
                true
            },
        );

        assert!(pass);
    }
}
