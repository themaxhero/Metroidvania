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

    #[derive(Clone)]
    struct RectCollider{
        position: Vec3,
        size: Vec3,
    }

    fn collision_2d_rect_vs_circle(
        rect_pos: Vec3,
        rect: RectCollider,
        circle_vel: Vec3,
        circle_radius: f32,
        circle_pos: Vec3
    ) -> (Vec3, bool) {
        let offseted_circle_pos = circle_pos - rect_pos;
        let half_sized: Vec3 = rect.size / 2.0;
        let nearest_point: Vec2 = Vec2::new(
            f32::clamp(offseted_circle_pos.x, -half_sized.x, half_sized.x),
            f32::clamp(offseted_circle_pos.y, -half_sized.y, half_sized.y),
        );
        let circle_potential_position = Vec2::new(
            offseted_circle_pos.x + circle_vel.x,
            offseted_circle_pos.y + circle_vel.y
        );
        let distance_ray = Vec2::new(
            nearest_point.x - circle_potential_position.x,
            nearest_point.y - circle_potential_position.y
        );
        let nearest_point_magnetude = f32::sqrt(
            nearest_point.x.powf(2.) + nearest_point.y.powf(2.)
        );
        let offseted_circle_pos_magnetude = f32::sqrt(
            circle_potential_position.x.powf(2.) + circle_potential_position.y.powf(2.)
        );
        let distance_ray_magnetude = f32::sqrt(
            distance_ray.x.powf(2.) + distance_ray.y.powf(2.)
        );
        let mut overlap = circle_radius - distance_ray_magnetude;
        if overlap.is_nan() {
            overlap = 0.0;
        }
        if overlap <= 0.0 {
            return (Vec3::new(0.0, 0.0, 0.0), false);
        }
        let diff = Vec2::normalize_or_zero(distance_ray) * overlap;
        let adjust = Vec3::new(
            circle_pos.x - diff.x,
            circle_pos.y - diff.y,
            circle_pos.z
        );
        return (adjust, true);

        // let collider_pos: Vec3 = rect_pos + rect.position;
        // let top_left: Vec3 = collider_pos - half_sized;
        // let bottom_right: Vec3 = collider_pos + half_sized;
        // let nearest_point: Vec2 = Vec2::new(
        //     f32::clamp(circle_pos.x, top_left.x, bottom_right.x),
        //     f32::clamp(circle_pos.y, top_left.y, bottom_right.y),
        // );
        // let distance_magnetude: f32 = match (distance_ray.x, distance_ray.y) {
        //     (0.0, 0.0) => 0.0,
        //     (x, y) => f32::sqrt(x.powf(2.0) + y.powf(2.0))
        // };
        // let mut overlap: f32 = circle_radius - distance_magnetude;
        // let unit_nearest_point: Vec2 = Vec2::new(
        //     match Some(nearest_point.x) {
        //         Some(x) if x < 0.0 => -1.0,
        //         Some(x) if x > 0.0 => 1.0,
        //         _ => 0.0,
        //     },
        //     match Some(nearest_point.y) {
        //         Some(y) if y < 0.0 => -1.0,
        //         Some(y) if y > 0.0 => 1.0,
        //         _ => 0.0,
        //     },
        // );
        // let circle_fixed_pos: Vec2 = Vec2::new(
        //     unit_nearest_point.x * overlap,
        //     unit_nearest_point.y * overlap,
        // );
        // let new_circle_pos: Vec3 = Vec3::new(
        //     circle_pos.x - circle_fixed_pos.x,
        //     circle_pos.y - circle_fixed_pos.y,
        //     circle_pos.z,
        // );
        // let collided: bool = distance_magnetude <= circle_radius;
        // let circle_position_adjustment = if collided {
        //     new_circle_pos
        // } else {
        //     Vec3::new(0.0, 0.0, 0.0)
        // };
        // println!("Half Sized:\n\tX: {}\n\tY: {}", half_sized.x, half_sized.y);
        // println!("Top Left:\n\tX: {}\n\tY: {}", top_left.x, top_left.y);
        // println!("Bottom Right:\n\tX: {}\n\tY: {}", bottom_right.x, bottom_right.y);
        // println!("Nearest Point:\n\tX: {}\n\tY: {}", nearest_point.x, nearest_point.y);
        // println!("Distance Magnetude:\n\tValue: {}", distance_magnetude);
        // println!("Overlap:\n\tValue: {}", overlap);
        // println!("Unit Nearest Point:\n\tX: {}\n\tY: {}", unit_nearest_point.x, unit_nearest_point.y);
        // println!("Circle Fixed Position:\n\tX: {}\n\tY: {}", circle_fixed_pos.x, circle_fixed_pos.y);
        // println!("New Circle Position:\n\tX: {}\n\tY: {}", new_circle_pos.x, new_circle_pos.y);
        // println!("Collider Position:\n\tX: {}\n\tY: {}", collider_pos.x, collider_pos.y);
        // println!("Collided:\n\tValue: {}", collided);
        // return (circle_position_adjustment, collided);
    }

    #[test]
    fn returns_true_when_collides_from_left() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(6.0, 0.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(11.0, 0.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_true_when_collides_from_right() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(-6.0, 0.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(-11.0, 0.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_true_when_collides_from_up() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(0.0, 6.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(0.0, 11.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_true_when_collides_from_down() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(0.0, -6.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(0.0, -11.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_true_when_collides_from_top_left() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(-5.0, 5.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(-5.0, 5.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_true_when_collides_from_top_right() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(5.0, 5.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(5.0, 5.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_true_when_collides_from_bottom_left() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(-5.0, -5.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(-5.0, -5.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_true_when_collides_from_bottom_right() {
        let rect_pos = Vec3::new(0.0, 0.0, 1.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 1.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 6.0;
        let circle_pos = Vec3::new(-5.0, 5.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone()), 
            (Vec3::new(-5.0, 5.0, 0.0), true)
        );
        let (adjustment, _) = collision_2d_rect_vs_circle(rect_pos.clone(), rect.clone(), circle_vel.clone(), circle_radius.clone(), circle_pos.clone());
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, adjustment),
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }

    #[test]
    fn returns_false_when_does_not_collide() {
        let rect_pos = Vec3::new(0.0, 0.0, 0.0);
        let rect = RectCollider {
            position: Vec3::new(0.0, 0.0, 0.0),
            size: Vec3::new(10.0, 10.0, 0.0)
        };
        let circle_vel = Vec3::new(0.0, 0.0, 0.0);
        let circle_radius = 1.0;
        let circle_pos = Vec3::new(100.0, 0.0, 0.0);
        assert_eq!(
            collision_2d_rect_vs_circle(rect_pos, rect, circle_vel, circle_radius, circle_pos), 
            (Vec3::new(0.0, 0.0, 0.0), false)
        );
    }
}

//-----------------------------------------------------------------------
// Collision Treatment
//-----------------------------------------------------------------------



fn main() {
    println!("Hello, world!");
}
