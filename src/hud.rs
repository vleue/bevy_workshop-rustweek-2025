use bevy::{prelude::*, time::Stopwatch};

use crate::{
    GameState,
    game::{Asteroid, LivesRemaining},
};

pub fn hud_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), display_hud)
        .add_systems(Update, update_hud.run_if(in_state(GameState::Game)));
}

#[derive(Resource)]
struct GameDuration(Stopwatch);

fn display_hud(mut commands: Commands) {
    commands.spawn((
        Text::default(),
        StateScoped(GameState::Game),
        children![
            TextSpan::new("Asteroids remaining: "),
            TextSpan::new("0"),
            TextSpan::new("\n"),
            TextSpan::new("Lives remaining: "),
            TextSpan::new("0"),
            TextSpan::new("\n"),
            TextSpan::new("Time: "),
            TextSpan::new("0")
        ],
    ));

    commands.insert_resource(GameDuration(Stopwatch::new()));
}

fn update_hud(
    time: Res<Time>,
    mut duration: ResMut<GameDuration>,
    text: Query<Entity, With<Text>>,
    mut writer: TextUiWriter,
    asteroids: Query<(), With<Asteroid>>,
    lives_remaining: Res<LivesRemaining>,
) -> Result {
    duration.0.tick(time.delta());

    let text = text.single()?;
    *writer.text(text, 2) = format!("{}", asteroids.iter().len());
    *writer.text(text, 5) = format!("{}", lives_remaining.0);
    *writer.text(text, 8) = format!("{:?}s", duration.0.elapsed().as_secs());

    Ok(())
}
