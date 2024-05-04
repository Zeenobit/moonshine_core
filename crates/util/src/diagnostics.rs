use bevy_ecs::prelude::*;

/// A trait similar to [`bevy_ecs::system::RunSystemOnce`], but it runs a system multiple times.
///
/// This is useful for testing multiple iterations of a system.
pub trait RunSystemLoop: Sized {
    /// Runs a system `n` times, returning a [`Vec`] of the outputs.
    ///
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use moonshine_util::diagnostics::RunSystemLoop;
    ///
    /// let mut world = World::new();
    /// let entities = world.run_system_loop(3, |mut commands: Commands| {
    ///     commands.spawn_empty();
    /// });
    ///
    /// assert_eq!(entities.len(), 3);
    /// assert_eq!(world.iter_entities().count(), 3);
    /// ```
    fn run_system_loop<T: IntoSystem<(), Out, Marker>, Out, Marker>(
        self,
        n: usize,
        system: T,
    ) -> Vec<Out> {
        self.run_system_loop_with(n, || (), system)
    }

    /// Runs a system `n` times with the given input source, returning a [`Vec`] of the outputs.
    ///
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use moonshine_util::diagnostics::RunSystemLoop;
    ///
    /// let mut world = World::new();
    /// let mut names = vec!["Alice", "Bob", "Charlie"];
    /// let entities = world.run_system_loop_with(
    ///     3,
    ///     || names.pop().unwrap().to_owned(),
    ///     |In(name): In<String>, mut commands: Commands| {
    ///         commands.spawn(Name::new(name));
    ///     });
    ///
    /// assert_eq!(entities.len(), 3);
    /// assert_eq!(world.query::<&Name>().iter(&mut world).count(), 3);
    /// ```
    fn run_system_loop_with<InputSource, T: IntoSystem<In, Out, Marker>, In, Out, Marker>(
        self,
        n: usize,
        input: InputSource,
        system: T,
    ) -> Vec<Out>
    where
        InputSource: FnMut() -> In;
}

impl RunSystemLoop for &mut World {
    fn run_system_loop_with<InputSource, T: IntoSystem<In, Out, Marker>, In, Out, Marker>(
        self,
        n: usize,
        mut input: InputSource,
        system: T,
    ) -> Vec<Out>
    where
        InputSource: FnMut() -> In,
    {
        let mut system: T::System = IntoSystem::into_system(system);
        system.initialize(self);
        let mut outs = Vec::new();
        for _ in 0..n {
            let out = system.run(input(), self);
            outs.push(out);
            system.apply_deferred(self);
        }
        outs
    }
}
