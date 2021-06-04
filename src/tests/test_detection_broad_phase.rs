use crate::{
    algebra::Vec2,
    detection_broad_phase::{detect_by_broad_phase, ShapeIndexPair},
    detection_narrow_phase::detect_collision_circle_and_circle,
    shapes::{Circle, Material, RigidBody, RigidBodyLike},
};

#[test]
fn test_detect_by_broad_phase_circle() {
    let zero = Vec2::new(0.0, 0.0);
    let circle1 = Circle::new(
        Material { restitution: 1.0 },
        RigidBody {
            position: Vec2::new(10.0, 10.0),
            mass: 1.0,
            velocity: zero,
            acceleration: zero,
        },
        10.0,
    );

    let mut circle2 = Circle::new(
        Material { restitution: 1.0 },
        RigidBody {
            position: Vec2::new(20.0, 10.0),
            mass: 1.0,
            velocity: zero,
            acceleration: zero,
        },
        10.0,
    );

    // 两圆相交
    {
        let circles = vec![&circle2, &circle1];
        let results = detect_by_broad_phase(&circles);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], ShapeIndexPair(0, 1));
    }

    // 两圆相切：相切不能算碰撞，无论宽窄检测，相切无形变，等于没有施力，故不需要处理碰撞冲突。
    {
        circle2.rigid_body_mut().position = Vec2::new(30.0, 10.0);
        let circles = vec![&circle1, &circle2];
        let results = detect_by_broad_phase(&circles);

        assert_eq!(results.len(), 0);
    }

    // 两圆相离
    {
        circle2.rigid_body_mut().position = Vec2::new(31.0, 10.0);
        let circles = vec![&circle1, &circle2];
        let results = detect_by_broad_phase(&circles);

        assert_eq!(results.len(), 0);
    }

    // 宽检测通过，窄检测不通过
    {
        circle2.rigid_body_mut().position = Vec2::new(25.0, 25.0);
        let circles = vec![&circle2, &circle1];
        let results = detect_by_broad_phase(&circles);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], ShapeIndexPair(0, 1));
        assert!(detect_collision_circle_and_circle(&circle1, &circle2).is_none());
    }
}
