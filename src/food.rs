use bevy::prelude::*;
use rand::Rng;
use crate::*;
use crate::collider::Collider;

const FOOD_COLOR: Color = Color::rgb(255.0, 0.0, 0.0);
const FOOD_SCALE: f32 = SCALE / 2.0;

#[derive(Component)]
pub struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_food)
            .add_systems(Update, check_eaten);
    }
}

pub fn food_bundle() -> (SpriteBundle, Food, Collider){
            (SpriteBundle{
                transform: Transform{
                    translation: Vec3::new(get_random_cord(), get_random_cord(), 0.0),
                    scale: Vec3::new(FOOD_SCALE, FOOD_SCALE, 0.0),
                    ..default()
                },
                sprite: Sprite{
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            }, 
            Food,
            Collider,
)}

pub fn spawn_food(mut commands: Commands){
    commands.spawn(food_bundle());
}

pub fn check_eaten(mut er_food: EventReader<FoodAte>, mut commands: Commands, query: Query<Entity, With<Food>>){
    for _ in er_food.read(){
        for e in &query{
            commands.entity(e).despawn();
            commands.spawn(food_bundle());
        }
    }
}

fn get_random_cord() -> f32{
    let mut rng = rand::thread_rng();
    return rng.gen_range(-(COLS_ROWS / 2.0)..(COLS_ROWS / 2.0)).floor() * SCALE;
}
