use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::quad,
        transform::components::{lookat_target, translation},
    },
    prelude::*,
};

#[main]
pub fn main() {
    let is_main_package = entity::wait_for_component(entity::resources(), main_package_id()).await
        == Some(packages::this::entity());

    if is_main_package {
        PerspectiveInfiniteReverseCamera {
            optional: PerspectiveInfiniteReverseCameraOptional {
                aspect_ratio_from_window: Some(entity::resources()),
                main_scene: Some(()),
                translation: Some(Vec3::ONE * 5.),
                ..default()
            },
            ..PerspectiveInfiniteReverseCamera::suggested()
        }
        .make()
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

        let zombie = Entity::new()
            .with_merge(Transformable {
                local_to_world: Default::default(),
                optional: Default::default(),
            })
            .with(prefab_from_url(), assets::url("Data/Units/Zombie1.x"))
            .spawn();

        let idle =
            PlayClipFromUrlNodeRef::new(assets::url("Data/Units/Zombie1.x/animations/Run1.anim"));
        let anim_player = AnimationPlayerRef::new(idle);
        entity::add_component(zombie, apply_animation_player(), anim_player.0);
    }
}
