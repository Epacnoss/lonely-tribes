use amethyst::core::{
    ecs::{Join, Read, System, WriteStorage},
    Time, Transform,
};
use lonely_tribes_animations::{
    animation::Animator, data::AnimationData, movement::MovementAnimationData,
    rotation::RotationAnimationData,
};
use lonely_tribes_components::tile_transform::TileTransform;
use lonely_tribes_lib::{
    states_util::{CAMERA_BASE_HEIGHT, CAMERA_BASE_WIDTH, CAMERA_DIMENSIONS},
    HEIGHT, TILE_WIDTH_HEIGHT,
};

/// System to turn TileTransforms into Transforms
pub struct UpdateTileTransforms;

///Offset x to have tile anchored to centre rather than corner.
pub const TILE_WIDTH: f32 = TILE_WIDTH_HEIGHT as f32 / 2.0;
///Offset y to have tile anchored to centre rather than corner.
pub const TILE_HEIGHT: f32 = TILE_WIDTH_HEIGHT as f32 / 2.0;

impl<'s> System<'s> for UpdateTileTransforms {
    type SystemData = (
        WriteStorage<'s, TileTransform>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Animator<MovementAnimationData>>,
        WriteStorage<'s, Animator<RotationAnimationData>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (tiles, mut transforms, mut movement_animators, mut rotation_animators, time): Self::SystemData,
    ) {
        let dims = *CAMERA_DIMENSIONS;
        let wide_x_offset = (dims.0 - CAMERA_BASE_WIDTH) / 2.0;
        let wide_y_offset = (dims.1 - CAMERA_BASE_HEIGHT) / 2.0;

        for (tile, trans) in (&tiles, &mut transforms).join() {
            let old_z = trans.translation().z;
            let x = tile.x as f32 * TILE_WIDTH_HEIGHT as f32
                + TILE_WIDTH
                + tile.x_offset as f32
                + wide_x_offset;
            let y = (HEIGHT as f32 - tile.y as f32) * TILE_WIDTH_HEIGHT as f32
                - TILE_HEIGHT
                - tile.y_offset as f32
                + wide_y_offset;

            trans.set_translation_xyz(x, y, old_z);
        }

        for (trans, anim_cmp) in (&mut transforms, &mut movement_animators).join() {
            if anim_cmp.anim_is_done() {
                anim_cmp.finish();
            } else if let Some(anim) = &mut anim_cmp.animation_data {
                anim.add_time(time.delta_seconds());

                //Translation
                let start = anim.start;
                let (xo, yo) = anim.get_current();

                let x = (((start.x as f32) - xo) * TILE_WIDTH_HEIGHT as f32 + TILE_WIDTH)
                    + wide_x_offset;
                let y = ((HEIGHT as f32 - start.y as f32) + yo) * TILE_WIDTH_HEIGHT as f32
                    - TILE_HEIGHT
                    + wide_y_offset;
                let z = trans.translation().z;

                trans.set_translation_xyz(x, y, z);
            }
        }
        for (trans, anim_cmp) in (&mut transforms, &mut rotation_animators).join() {
            if anim_cmp.anim_is_done() {
                anim_cmp.finish();
                trans.set_rotation_2d(0.0);
            } else if let Some(anim) = &mut anim_cmp.animation_data {
                anim.add_time(time.delta_seconds());
                trans.set_rotation_2d(anim.get_current());
            }
        }
    }
}

impl UpdateTileTransforms {
    ///Convert a TileTransform to a Transform on Screen
    #[allow(dead_code)]
    pub fn tile_to_transform(tile: TileTransform, z: f32) -> Transform {
        let mut trans = Transform::default();
        let (x, y) = Self::tile_to_xyz(tile);
        trans.set_translation_xyz(x, y, z);
        trans
    }
    ///Convert a TileTransform to an XYZ for a Transform on Screen
    pub fn tile_to_xyz(tile: TileTransform) -> (f32, f32) {
        let x = tile.x as f32 * TILE_WIDTH_HEIGHT as f32 + TILE_WIDTH;
        let y = (HEIGHT - tile.y) as f32 * TILE_WIDTH_HEIGHT as f32 - TILE_HEIGHT;
        (x, y)
    }
}
