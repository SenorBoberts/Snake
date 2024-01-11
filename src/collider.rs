use bevy::prelude::*;
use crate::FoodAte;
use crate::snake::Snake;
use crate::food::Food;

#[derive(Component)]
pub struct Collider;

pub fn check_collision(mut ev_food: EventWriter<FoodAte>, snake: Query<&Transform, With<Snake>>, food: Query<&Transform, With<Food>>){
    for f in &food{
        for s in &snake{
            if f.translation == s.translation{
                ev_food.send(FoodAte);
            } 
        }
    }

}
