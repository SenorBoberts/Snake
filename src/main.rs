use bevy::prelude::*;

mod snake;
mod food;
mod collider;

pub const SCALE: f32 = 20.0;
pub const COLS_ROWS: f32 = 19.0;
const TIME_INTERVAL: f64 = 0.15;

#[derive(Event)]
pub struct FoodAte;

#[derive(Resource)]
pub struct Score(i32);

#[derive(Resource)]
pub struct Segs(Vec<Entity>);

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
        .add_systems(Startup, (setup, snake::spawn_snake, food::spawn_food))
        .add_systems(FixedUpdate, (snake::move_snake, snake::check_inbounds, collider::check_collision).chain())
        .add_systems(Update, (snake::update_direction, food::check_eaten, snake::grow, bevy::window::close_on_esc))
        .insert_resource(Time::<Fixed>::from_seconds(TIME_INTERVAL))
        .insert_resource(Score(0))
        .insert_resource(Segs(vec!()))
        .add_event::<FoodAte>()
        .run()
}

fn setup(mut commands: Commands){
    //spawn camera
    commands.spawn(Camera2dBundle::default());
}

fn update(mut commands: Commands){

}

