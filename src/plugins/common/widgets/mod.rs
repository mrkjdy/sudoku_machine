use bevy::{ecs::system::EntityCommands, prelude::*};

pub mod dropdown;
pub mod text_input;

pub trait Spawn {
    fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands<'_>;
}

impl<'a> Spawn for ChildBuilder<'a> {
    fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
        ChildBuild::spawn(self, bundle)
    }
}

pub trait Spawnable {
    fn spawn<'a, S: Spawn>(&self, spawner: &'a mut S) -> EntityCommands<'a> {
        self.spawn_with_components(spawner, ())
    }

    fn spawn_with_components<'a, S: Spawn>(
        &self,
        spawner: &'a mut S,
        components: impl Bundle,
    ) -> EntityCommands<'a>;
}
