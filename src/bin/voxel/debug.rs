
pub mod debug {
    use bevy::prelude::*;
    use bevy::render::primitives::Aabb;

    #[derive(Component)]
    pub struct ShowAxes;


    pub fn draw_axes(mut gizmos: Gizmos, query: Query<(&Transform, &Aabb), With<ShowAxes>>) {
        for (&transform, &aabb) in &query {
            let length = aabb.half_extents.length();
            gizmos.axes(transform, length);
        }
    }
}