use crate::components::{Ball, Player, Position};
use crate::stares::GameState;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use crate::systems::paddle::PADDLE_SPEED;
use crate::{components::Velocity, systems};
use crate::ui::menu_ui::{cleanup, handle_menu_ui, start};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Velocity>();
        app.init_state::<GameState>();

        // 注册其他需要的类型
        //    .register_type::<Position>()
        //    .register_type::<Paddle>()
        app.add_systems(Startup,  systems::setup::spawn_camera);
        app.add_systems(OnEnter(GameState::Menu), start);
        app.add_systems(OnExit(GameState::Menu), cleanup);




        app.add_systems(Update, handle_menu_ui.run_if(in_state(GameState::Menu)));
        app.add_systems(
            // OnEnter(GameState::Playing),
            Startup,
            (
                systems::setup::setup,
               
                systems::ball::spawn_ball,
                systems::paddle::spawn_paddles,
                systems::gutters::spawn_gutters,
            )
                .chain(),
        );
        // app.add_systems(FixedPreUpdate, handle_player_input);
        app.add_systems(
            FixedUpdate,
            (
                handle_player_input,
                systems::ball::move_ball,
                systems::paddle::ai_paddle.after(systems::ball::move_ball),
                systems::setup::project_positions.after(systems::ball::move_ball),
                systems::paddle::handle_collisions.after(systems::ball::move_ball),
                systems::paddle::move_paddles.after(systems::ball::move_ball),
                game_over,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut Velocity, With<Player>>,
) {
    // info!("Player input: {:?}", keyboard_input);
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        paddle_velocity.0.y = PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        paddle_velocity.0.y = -PADDLE_SPEED;
    } else {
        paddle_velocity.0.y = 0.;
    }
}

fn game_over(
    mut commands: Commands,
    ball: Single<(&mut Position, &mut Velocity), With<Ball>>,
    win: Single<&Window>,
) {
    // win.resolution.width();
    let (mut ball, mut vle) = ball.into_inner();

    if ball.0.x > win.resolution.width() / 2. || ball.0.x < -win.resolution.width() / 2. {
        ball.0 = Vec2::new(0., 0.);
        vle.0.y /= 2.;
        vle.0.x = -vle.0.x;
        // entity
        //  commands.entity(entity).despawn();
        info!("Game Over!");
        commands.insert_resource(NextState::Pending(GameState::Menu));
    }


}

pub struct GameStatesPlugins;
impl  PluginGroup for GameStatesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // .add(FpsPlugin)
            // .add()
    }
}

// start
