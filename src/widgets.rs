use bevy::{ecs::system::IntoObserverSystem, prelude::*};

pub fn button<O, E, B, M>(text: impl Into<String>, observer: O) -> impl Bundle
where
    O: IntoObserverSystem<E, B, M> + Send + Sync,
    E: EntityEvent,
    B: Bundle,
{
    let text: String = text.into();

    let system = IntoObserverSystem::into_system(observer);

    (
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::axes(px(12.), px(8.)),
                        border: px(2.).all(),

                        ..default()
                    },
                    Button,
                    BorderRadius::all(px(5.)),
                    BackgroundColor(Color::WHITE),
                    BorderColor::all(Color::BLACK),
                    children![(Text::new(text), TextColor(Color::BLACK))],
                ))
                .observe(system);
        })),
    )
}
