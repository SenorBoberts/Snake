use bevy::prelude::*;

mod snake;
mod food;
mod collider;

pub const SCALE: f32 = 20.0;
pub const COLS_ROWS: f32 = 19.0;
const TIME_INTERVAL: f64 = 0.15;

#[derive(Event)]
pub struct FoodAte;

#[derive(Event)]
pub struct GameOver;

#[derive(Resource)]
pub struct Score(i32);

#[derive(Resource)]
pub struct Segs(Vec<Entity>);

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState{
    #[default]
    Alive, 
    Dead
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
                     .set(WindowPlugin{
                         primary_window: Some(Window{
                             title: "Snake".into(),
                             resolution: (COLS_ROWS * SCALE, COLS_ROWS * SCALE).into(),
                             resizable: false,
                             ..default()
                         }),
                         ..default()
                     }))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, collider::check_collision)
        .add_systems(Update, (game_over, bevy::window::close_on_esc))
        .add_plugins((snake::SnakePlugin, food::FoodPlugin))
        .insert_resource(Time::<Fixed>::from_seconds(TIME_INTERVAL))
        .insert_resource(Score(0))
        .insert_resource(Segs(vec!()))
        .add_event::<FoodAte>()
        .add_event::<GameOver>()
        .add_state::<crate::GameState>()
        .run()
}

fn setup(mut commands: Commands){
    //spawn camera
    commands.spawn(Camera2dBundle::default());
}

fn game_over(mut commands: Commands, mut e: EventReader<GameOver>, mut query: Query<Entity, Without<Window>>){
    if let Some(_) = e.read().into_iter().next(){
        for mut e in &mut query{
            commands.entity(e).despawn();
        }
    }  
}

