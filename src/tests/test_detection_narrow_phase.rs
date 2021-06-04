use crate::{
    algebra::{equals_float, Vec2, FLOADT_TOLERANCE},
    detection_narrow_phase::detect_collision_circle_and_circle,
    shapes::{Circle, Material, RigidBody, RigidBodyLike},
};

#[test]
fn test_detect_collision_circle_and_circle() {
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
    let mut circle2 = Circle::new(
        Material { restitution: 1.0 },
        RigidBody {
            mass: 1.0,
            position: Vec2::new(30.0, 10.0),
            velocity: zero,
            acceleration: zero,
        },
        10.0,
    );

    // 两圆相切，相切不能算碰撞，无论宽窄检测，相切无形变，等于没有施力，故不需要处理碰撞冲突。
    assert!(detect_collision_circle_and_circle(&circle1, &circle2).is_none());

    // 两圆临界相交：穿模深度为 FLOADT_TOLERANCE 时，应当能够被检测出来。
    circle2.rigid_body_mut().position = Vec2::new(30.0 - FLOADT_TOLERANCE, 10.0);
    let result = detect_collision_circle_and_circle(&circle1, &circle2);
    assert!(result.is_some());
    let info = result.unwrap();
    assert!(info.penetration.abs() > 0.0);
    assert!(!equals_float(info.penetration, 0.0));
    assert_eq!(info.normal, Vec2::new(1.0, 0.0));
}
