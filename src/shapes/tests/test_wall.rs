use rand::random;

use crate::{
    algebra::{Float, Vec2, FLOADT_TOLERANCE},
    shapes::{Circle, Collider, Material, Orientation, RigidBody, RigidBodyLike, Wall},
};

#[test]
fn test_collide_circle_and_wall() {
    // 创建一个位置在 (10, 10) 点，半径略大于矩形框 [(0, 0), (20, 20)] 的圆，
    // 下面测试的墙体刚好等于该矩形框，这样，该圆将与四个朝向的墙体均发生碰撞，方便测试。
    fn get_circle() -> Circle {
        Circle::new(
            Material { restitution: 1.0 },
            RigidBody {
                mass: 1.0,
                position: Vec2::new(10.0, 10.0),
                velocity: Vec2::new(0.0, 0.0),
                acceleration: Vec2::new(0.0, 0.0),
            },
            10.0 + FLOADT_TOLERANCE,
        )
    }

    fn random_float(a: Float, b: Float) -> Float {
        (b - a) * random::<Float>() + a
    }

    fn test_circle_and_wall(wall_bound: Float, wall_orientation: Orientation) {
        let mut circle = get_circle();
        let v = Vec2::new(random_float(-100.0, 100.0), random_float(-100.0, 100.0));
        circle.rigid_body_mut().velocity = v;
        let mut wall = Wall::new(Material { restitution: 1.0 }, wall_bound, wall_orientation);

        // 第一次碰撞：与朝向平行方向上的速度应等大反向，而垂直方向上的速度则不变。
        circle.collide_with(&mut wall);
        match wall_orientation {
            Orientation::Left | Orientation::Right => {
                assert_eq!(circle.rigid_body().velocity, Vec2::new(-v.x, v.y))
            }
            Orientation::Top | Orientation::Bottom => {
                assert_eq!(circle.rigid_body().velocity, Vec2::new(v.x, -v.y))
            }
        }

        // 第二次碰撞：第一次碰撞应当完全解决碰撞冲突，故第二次碰撞前后速度保持不变。
        let v_before = circle.rigid_body().velocity;
        circle.collide_with(&mut wall);
        assert_eq!(circle.rigid_body().velocity, v_before);
    }

    for _ in 0..10000 {
        test_circle_and_wall(0.0, Orientation::Left);
        test_circle_and_wall(20.0, Orientation::Top);
        test_circle_and_wall(20.0, Orientation::Right);
        test_circle_and_wall(0.0, Orientation::Bottom);
    }
}
