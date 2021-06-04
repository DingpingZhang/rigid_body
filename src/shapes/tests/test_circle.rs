use rand::random;

use crate::{
    algebra::{equals_float, Vec2, FLOADT_TOLERANCE},
    detection_narrow_phase::detect_collision_circle_and_circle,
    shapes::{Bounded, Circle, Material, RigidBody, RigidBodyLike, Collide},
};

#[test]
fn test_collide_circle_speed_exchange() {
    // 两质量相等的小球，对心完全弹性碰撞，末状态：速度交换。
    let (mut circle1, mut circle2) = get_two_intersecting_circle();
    let v1 = Vec2::new(32.0, 0.0);
    let v2 = Vec2::new(-10086.0, 0.0);

    circle1.rigid_body_mut().velocity = v1;
    circle2.rigid_body_mut().velocity = v2;

    circle1.collide_with(&mut circle2);
    assert_ne!(circle1.rigid_body().velocity, circle2.rigid_body().velocity);
    assert_eq!(circle1.rigid_body().velocity, v2);
    assert_eq!(circle2.rigid_body().velocity, v1);
}

#[test]
fn test_collide_circle_moving_away() {
    // 正在远离的两小球，不应该被处理碰撞。
    let (mut circle1, mut circle2) = get_two_intersecting_circle();
    let v1 = Vec2::new(-32.0, -21.0);
    let v2 = Vec2::new(10086.0, 42.0);

    circle1.rigid_body_mut().velocity = v1;
    circle2.rigid_body_mut().velocity = v2;

    circle1.collide_with(&mut circle2);
    assert_eq!(circle1.rigid_body().velocity, v1);
    assert_eq!(circle2.rigid_body().velocity, v2);
}

#[test]
fn test_collide_circle_mass_infinity() {
    // 两小球，对心完全弹性碰撞，一球质量无穷大，另一球质量有限，末状态：有限质量的小球倒飞出去，无穷质量的小球速度维持静止。
    let (mut circle1, mut circle2) = get_two_intersecting_circle();
    // 该场景中，无穷质量的小球速度必为 0，否则不符合常识。
    let v1 = Vec2::new(0.0, 0.0);
    let v2 = Vec2::new(-42.0, 0.0);
    circle1.rigid_body_mut().velocity = v1;
    circle1.rigid_body_mut().mass = f64::INFINITY;

    circle2.rigid_body_mut().velocity = v2;

    circle2.collide_with(&mut circle1);
    assert_eq!(circle1.rigid_body().velocity, v1);
    assert_eq!(circle2.rigid_body().velocity, Vec2::new(-v2.x, -v2.y));
}

#[test]
fn test_collide_circle_momentum_and_energy_conservation() {
    fn random_f64(a: f64, b: f64) -> f64 {
        (b - a) * random::<f64>() + a
    }

    for _ in 0..100000 {
        let (mut circle1, mut circle2) = get_two_intersecting_circle();
        circle1.rigid_body_mut().velocity =
            Vec2::new(random_f64(-100.0, 100.0), random_f64(-100.0, 100.0));
        circle1.rigid_body_mut().mass = random_f64(FLOADT_TOLERANCE, 100.0);
        circle2.rigid_body_mut().velocity =
            Vec2::new(random_f64(-100.0, 100.0), random_f64(-100.0, 100.0));
        circle2.rigid_body_mut().mass = random_f64(FLOADT_TOLERANCE, 100.0);
        let circle2_x = random_f64(
            circle1.bound_left() - circle2.radius,
            circle1.bound_right() + circle2.radius,
        );
        let sign = if random() { 1.0 } else { -1.0 };
        let circle2_y = sign
            * ((circle1.radius + circle2.radius - FLOADT_TOLERANCE).powi(2)
                - (circle2_x - circle1.rigid_body().position.x).abs().powi(2))
            .sqrt()
            + circle1.rigid_body().position.y;
        circle2.rigid_body_mut().position = Vec2::new(circle2_x, circle2_y);

        let momentum_before = get_momentum(circle1.rigid_body(), circle2.rigid_body());
        let kinetic_energy_before = get_kinetic_energy(circle1.rigid_body(), circle2.rigid_body());

        assert!(detect_collision_circle_and_circle(&circle1, &circle2).is_some());
        circle1.collide_with(&mut circle2);

        let momentum_after = get_momentum(circle1.rigid_body(), circle2.rigid_body());
        let kinetic_energy_after = get_kinetic_energy(circle1.rigid_body(), circle2.rigid_body());

        assert_eq!(momentum_before, momentum_after);
        assert!(equals_float(kinetic_energy_before, kinetic_energy_after));
    }
}

// Helper Functions

fn get_two_intersecting_circle() -> (Circle, Circle) {
    let zero = Vec2::new(0.0, 0.0);

    let circle1 = Circle::new(
        Material { restitution: 1.0 },
        RigidBody {
            mass: 1.0,
            position: Vec2::new(10.0, 10.0),
            velocity: zero,
            acceleration: zero,
        },
        10.0,
    );
    let circle2 = Circle::new(
        Material { restitution: 1.0 },
        RigidBody {
            mass: 1.0,
            position: Vec2::new(30.0, 10.0),
            velocity: zero,
            acceleration: zero,
        },
        10.0 + FLOADT_TOLERANCE,
    );

    (circle1, circle2)
}

fn get_momentum(p1: &RigidBody, p2: &RigidBody) -> Vec2 {
    p1.velocity * p1.mass + p2.velocity * p2.mass
}

fn get_kinetic_energy(p1: &RigidBody, p2: &RigidBody) -> f64 {
    (p1.mass * p1.velocity.length_squared() + p2.mass * p2.velocity.length_squared()) / 2.0
}
