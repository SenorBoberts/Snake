use bevy::prelude::*;
use crate::*;
use crate::collider::Collider;

const SNAKE_COLOR: Color = Color::rgb(255.0, 255.0, 255.0);

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct Tail(Vec<Entity>);

#[derive(Component)]
pub enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_snake)
            .add_systems(FixedUpdate, (check_inbounds, move_snake)) 
            .add_systems(Update, (update_direction, grow));
    }
}

pub fn spawn_snake(mut commands: Commands){
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(SCALE - 2.0,SCALE - 2.0,0.0),
                ..default()
            },
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        },
        Snake,
        Direction::RIGHT,
        Collider, 
        ));
}

pub fn spawn_segment(mut commands: Commands, x: f32, y: f32) -> Entity{
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::new(SCALE - 2.0,SCALE - 2.0,0.0),
            ..default()
        },
        sprite: Sprite {
            color: SNAKE_COLOR,
            ..default()
        },
        ..default()
    }, 
    SnakeTail,
    )).id()
}

//This is far simpler than nice smooth movement, its a retro game
pub fn move_snake(mut commands: Commands, mut query: Query<(&mut Transform, &Direction), With<Snake>>, mut segs_q: Query<&mut Transform, Without<Snake>>, mut v: ResMut<Segs>){
    if let Some((mut transform, direction)) = query.iter_mut().next(){
        move_tail(commands, transform.translation.x, transform.translation.y, &mut v, &mut segs_q);
        match direction{
            Direction::UP => {transform.translation.x += 0.0; transform.translation.y += SCALE},
            Direction::DOWN => {transform.translation.x += 0.0; transform.translation.y += -SCALE},
            Direction::LEFT => {transform.translation.x += -SCALE; transform.translation.y += 0.0},
            Direction::RIGHT => {transform.translation.x += SCALE; transform.translation.y += 0.0},
        }
    }
}

pub fn move_tail(mut commands: Commands, x: f32, y: f32, v: &mut ResMut<Segs>, q: &mut Query<&mut Transform, Without<Snake>>){
    if v.0.len() > 0{
        let i = v.0.len() - 1;
        commands.entity(v.0[i]).despawn();
        v.0.remove(i);
        v.0.insert(0, spawn_segment(commands, x, y));
    }
}

pub fn update_direction(key: Res<Input<KeyCode>>, mut query: Query<&mut Direction, With<Snake>>){
    for mut direction in &mut query{
        if key.pressed(KeyCode::Up) {*direction = Direction::UP} 
        if key.pressed(KeyCode::Down) {*direction = Direction::DOWN} 
        if key.pressed(KeyCode::Left) {*direction = Direction::LEFT} 
        if key.pressed(KeyCode::Right) {*direction = Direction::RIGHT} 
    }
}

pub fn grow(commands: Commands, mut e: EventReader<FoodAte>, mut score: ResMut<Score>, mut v: ResMut<Segs>){
    if e.read().next().is_some(){
       v.0.push(spawn_segment(commands, 1000.0, 1000.0)); //spawn off screen 
       score.0 += 1;
    }
}

pub fn check_inbounds(query: Query<&Transform, With<Snake>>, mut e: EventWriter<GameOver>){
    for transform in &query{
        if transform.translation.x < -SCALE * (COLS_ROWS / 2.0) || transform.translation.x > SCALE * (COLS_ROWS / 2.0) ||
            transform.translation.y < -SCALE * (COLS_ROWS / 2.0) || transform.translation.y > SCALE * (COLS_ROWS / 2.0){
                e.send(GameOver);
        }
    }
}
