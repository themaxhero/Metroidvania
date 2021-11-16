//use bevy::ecs::schedule::RunOnce;
use bevy::prelude::*;
//use bevy::render::pass::ClearColor;

//-----------------------------------------------------------------------
// Collision Detection
//-----------------------------------------------------------------------

struct RectCollider{
    position: Vec3,
    size: Vec3,
}


fn collision_3d_rect_vs_circle(
    _rect_pos: Vec3,
    _rect: RectCollider,
    _rect_velocity: Vec3,
    _circle_radius: f32,
    _circle_pos: Vec3
) -> (Vec3, bool) {
   (Vec3::new(0.0, 0.0, 0.0), false) 
}

#[cfg(test)]
mod tests {
use bevy::prelude::*;

struct RectCollider{
    position: Vec3,
    size: Vec3,
}

fn collision_2d_rect_vs_circle(
    rect_pos: Vec3,
    rect: RectCollider,
    _rect_vel: Vec3,
    circle_radius: f32,
    circle_pos: Vec3
) -> (Vec3, bool) {
    let half_sized: Vec3 = rect.size / 2.0;
    let collider_pos: Vec3 = rect_pos + rect.position;
    let nearest_point: Vec2 = Vec2::new(
        f32::clamp(circle_pos.x, collider_pos.x - half_sized.x, collider_pos.x + half_sized.x),
        f32::clamp(circle_pos.y, collider_pos.y - half_sized.y, collider_pos.y + half_sized.y),
    );

    let distance_magnetude: f32 = f32::sqrt(
        f32::powf(nearest_point.x, 2.0) + f32::powf(nearest_point.y, 2.0)
    );

    let overlap: f32 = circle_radius - distance_magnetude;

    let unit_nearest_point: Vec2 = Vec2::new(
        nearest_point.x / f32::abs(nearest_point.x),
        nearest_point.y / f32::abs(nearest_point.y),
    );

    let circle_fixed_pos: Vec2 = Vec2::new(
        unit_nearest_point.x * -1.0 * overlap,
        unit_nearest_point.y * -1.0 * overlap,
    );

    let new_circle_pos: Vec3 = Vec3::new(
        circle_pos.x + circle_fixed_pos.x,
        circle_pos.y + circle_fixed_pos.y,
        circle_pos.z,
    );

    let collided: bool = distance_magnetude <= circle_radius;
    return (new_circle_pos, collided);
}
    #[test]
    fn returns_true_when_collides() {
        assert_eq!(
            collision_2d_rect_vs_circle(
                Vec3::new(                           // rect_pos: Vec3,
                    0.0,
                    0.0,
                    1.0,
                ),
                RectCollider {                   // rect: RectCollider,
                    position: Vec3::new(
                        0.0,
                        0.0,
                        1.0,
                    ),
                    size: Vec3::new(
                        10.0,
                        10.0,
                        0.0,
                    )
                },  
                Vec3::new(                           // rect_vel: Vec3,
                    0.0,
                    0.0,
                    0.0,
                ),   
                6.0,                             // circle_radius: f32,
                Vec3::new(                           // circle_pos: Vec3
                    1.0,
                    1.0,
                    1.0,
                ),
            ), 
            (
                Vec3::new( 
                    0.0,
                    0.0,
                    0.0,
                ),
                true
            )
        );
    }
    #[test]
    fn returns_false_when_does_not_collide() {
        assert_eq!(
            collision_2d_rect_vs_circle(
                Vec3::new(                           // rect_pos: Vec3,
                    0.0,
                    0.0,
                    0.0,
                ),
                RectCollider {                   // rect: RectCollider,
                    position: Vec3::new(
                        0.0,
                        0.0,
                        0.0,
                    ),
                    size: Vec3::new(
                        0.0,
                        0.0,
                        0.0,
                    )
                },  
                Vec3::new(                           // rect_vel: Vec3,
                    0.0,
                    0.0,
                    0.0
                ),   
                0.0,                             // circle_radius: f32,
                Vec3::new(                           // circle_pos: Vec3
                    0.0,
                    0.0,
                    0.0,
                ),
            ), 
            (
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                true
            )
        );
    }
}

//-----------------------------------------------------------------------
// Collision Treatment
//-----------------------------------------------------------------------



fn main() {
    println!("Hello, world!");
}
