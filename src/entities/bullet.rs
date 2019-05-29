use amethyst::{
    core::{Transform},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, World},
    prelude::Builder,
    renderer::{SpriteRender, Transparent},
};

use crate::{
    SCALE,
    components::{Marine, Motion, TwoDimObject},
    resources::BulletResource,
};
use super::load_sprite_sheet;

pub fn init(world: &mut World) {
    let sprite_sheet = load_sprite_sheet(world, "sprites/bullet.png", "prefabs/bullet.ron");

    let bullet_resource = BulletResource {
        sprite_sheet: sprite_sheet,
    };

    world.add_resource(bullet_resource.clone());
}

pub fn spawn_bullet(entities: &Entities, bullet_resource: &ReadExpect<BulletResource>, shoot_start_position: f32, x_velocity: f32, lazy_update: &ReadExpect<LazyUpdate>) {
    let bullet_entity: Entity = entities.create();

    let mut transform = Transform::default();
    transform.set_scale(4. * SCALE, 4. * SCALE, 4. * SCALE);

    let mut two_dim_object = TwoDimObject::new(22. * 4. * SCALE, 22. * 4. * SCALE);
    two_dim_object.set_position(shoot_start_position, 80.);
    two_dim_object.update_transform_position(&mut transform);

    let sprite_render = SpriteRender {
        sprite_sheet: bullet_resource.sprite_sheet.clone(),
        sprite_number: 0,
    };

    lazy_update.insert(bullet_entity, sprite_render);
    lazy_update.insert(bullet_entity, two_dim_object);
    lazy_update.insert(bullet_entity, transform);
    lazy_update.insert(bullet_entity, Transparent);
}
