use bevy::prelude::*;

use crate::{
    combat::{
        phases::TransitionPhaseEvent,
        skills::{ExecuteSkillEvent, SkillExecutionQueue, SkillToExecute},
        CombatState,
    },
    spritesheet::{SpriteSheetAnimation, VFXSheet},
};

pub struct FXPlugin;

impl Plugin for FXPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    add_skill_vfx,
                    animate_skill_vfx,
                )
                    .in_set(CombatState::ExecuteSkills) // SkillsAnimation
            );
    }
}

#[derive(Component)]
pub struct SkillAnimation;

pub fn add_skill_vfx(
    skill_execution_queue: ResMut<SkillExecutionQueue>,
    vfx_sheet: Res<VFXSheet>,

    mut commands: Commands,
    skill_vfx_query: Query<Entity, With<SkillAnimation>>,

    mut transition_phase_event: EventWriter<TransitionPhaseEvent>,
) {
    // Wait for the Last VFX to finish...
    if skill_vfx_query.is_empty() {
        // TOTEST: One skill is animated once and in order
        if let Some(SkillToExecute {
            skill,
            caster: _caster,
            target,
        }) = skill_execution_queue.last()
        {
            // info!("{}. {:?} to {:?}", skill.initiative, caster, target);
            commands.entity(*target).with_children(|parent| {
                parent.spawn((
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(skill.vfx_index.start_index),
                        texture_atlas: vfx_sheet.0.clone(),
                        ..default()
                    },
                    Name::new("VFX"),
                    SkillAnimation,
                    // -- Animation --
                    SpriteSheetAnimation {
                        index: skill.vfx_index.clone(),
                        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    },
                ));
            });
        } else {
            transition_phase_event.send(TransitionPhaseEvent(CombatState::new_turn()));
        }
    }
}

/// At the end of the anim execute the skill
pub fn animate_skill_vfx(
    mut commands: Commands,
    time: Res<Time>,
    mut skill_vfx_query: Query<
        (Entity, &mut SpriteSheetAnimation, &mut TextureAtlasSprite),
        With<SkillAnimation>,
    >,

    mut execute_skill_event: EventWriter<ExecuteSkillEvent>,
) {
    for (vfx_id, mut animation, mut sprite) in &mut skill_vfx_query {
        // info!("VFX {:?} animation: {}", vfx_id, sprite.index);
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if sprite.index == animation.index.end_index {
                commands.entity(vfx_id).despawn();
                execute_skill_event.send(ExecuteSkillEvent);
            } else {
                sprite.index += 1;
            }
        }
    }
}
