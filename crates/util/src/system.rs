use bevy_ecs::prelude::*;

/// A [`System`] which checks if a given resource exists.
///
/// This is useful when used in combination with [`IntoSystemConfigs::run_if`].
///
/// # Example
/// ```
/// use bevy::prelude::*;
/// use moonshine_util::system::has_resource;
///
/// #[derive(Resource)]
/// struct R;
///
/// fn system(resource: Option<Res<R>>) {
///     assert!(resource.is_some());
/// }
///
/// let mut app = App::new();
/// app.add_plugins(MinimalPlugins);
/// app.add_systems(Update, system.run_if(has_resource::<R>));
/// app.update(); // If resource does't exist (it doesn't), system will panic!
/// ```
pub fn has_resource<T: Resource>(resource: Option<Res<T>>) -> bool {
    resource.is_some()
}

/// A [`System`] which checks if any event of a given type has been dispatched.
///
/// This is useful when used in combination with [`IntoSystemConfigs::run_if`].
///
/// # Example
/// ```
/// use bevy::prelude::*;
/// use moonshine_util::system::has_event;
///
/// #[derive(Event)]
/// struct E;
///
/// fn system(mut events: EventReader<E>) {
///     assert!(events.read().next().is_some());
/// }
///
/// let mut app = App::new();
/// app.add_plugins(MinimalPlugins).add_event::<E>();
/// app.add_systems(Update, system.run_if(has_event::<E>));
/// app.update(); // If event isn't dispatched (it wasn't), system will panic!
/// ```
pub fn has_event<T: Event>(events: EventReader<T>) -> bool {
    !events.is_empty()
}

/// A [`System`] which removes a resource from the world using [`Commands`].
///
/// # Example
/// ```
/// use bevy::prelude::*;
/// use moonshine_util::system::{remove_resource, has_resource};
///
/// #[derive(Resource)]
/// struct R;
///
/// let mut app = App::new();
/// app.insert_resource(R);
/// app.add_plugins(MinimalPlugins);
/// app.add_systems(Update, remove_resource::<R>.run_if(has_resource::<R>));
/// app.update(); // Resource is removed from the world.
///
/// assert!(!app.world.contains_resource::<R>());
/// ```
pub fn remove_resource<T: Resource>(mut commands: Commands) {
    commands.remove_resource::<T>();
}

/// A [`System`] which removes a resource from the world immediately.
///
/// # Example
/// ```
/// use bevy::prelude::*;
/// use moonshine_util::system::{remove_resource_immediate, has_resource};
///
/// #[derive(Resource)]
/// struct R;
///
/// let mut app = App::new();
/// app.insert_resource(R);
/// app.add_plugins(MinimalPlugins);
/// app.add_systems(Update, remove_resource_immediate::<R>.run_if(has_resource::<R>));
/// app.update(); // Resource is removed from the world.
///
/// assert!(!app.world.contains_resource::<R>());
/// ```
pub fn remove_resource_immediate<T: Resource>(world: &mut World) {
    world.remove_resource::<T>();
}
