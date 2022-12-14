#![allow(dead_code)]

use bevy::prelude::*;
use bevy_ggrs::Rollback;
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, PlayerHandle};

use crate::players::info;
use crate::animation::{animation_helper, play};

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct BoxInput {
    pub inp: u8, // List of inputs: up, down, left, right.
}

// Handles one movement.
pub fn input(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> BoxInput {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::W) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input |= INPUT_RIGHT;
    }

    BoxInput { inp: input }
}

pub fn animate_moving_player(
    animations: Res<play::CharacterAnimations>,
    // assets: Res<Assets<AnimationClip>>,
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
    mut query: Query<
        (
            Entity,
            &Children,
            &mut Transform,
            &info::Player,
            &animation_helper::AnimationHelper,
        ),
        With<Rollback>,
    >,
) {
    for (e, children, mut t, p, helper) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;

        // W
        if input & INPUT_UP != 0 && input & INPUT_DOWN == 0 {
            //println!("pressed W");
            //println!("{}", t.translation);

            //check that the shooter's parent entity's helper entity has the same id as the animation_player entity

            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[1].clone_weak());
                    //println!("Player animation W");
                    t.translation.z += 0.1;

                    // let a: &Assets<AnimationClip>;
                    // let animation_clip = Assets::get(&animations.0[1].clone_weak());
                }
            }
        }
        // S
        if input & INPUT_UP == 0 && input & INPUT_DOWN != 0 {
            //println!("pressed S");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[2].clone_weak());
                    //println!("Player animation S");
                    t.translation.z -= 0.1;
                }
            }
        }
        // A
        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            //println!("pressed A");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[3].clone_weak());
                    //println!("Player animation A");
                    t.translation.x += 0.1;
                }
            }
        }
        // D
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            //println!("pressed D");
            for (player_ent, mut player) in &mut player {
                if helper.player_entity.id() == player_ent.id() {
                    player.play(animations.0[4].clone_weak());
                    //println!("Player animation D");
                    t.translation.x -= 0.1;
                }
            }
        }
    }
}

