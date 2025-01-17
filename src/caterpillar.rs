use std::f32::consts::PI;
use bevy::prelude::*;
use avian3d::prelude::*;

pub struct CaterpillarPlugin;
impl Plugin for CaterpillarPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, moving)
        .add_observer(pull_changed)
        ;
    }
}

// ---

#[derive(Component)]
pub struct Hinge;

#[derive(Resource, Clone)]
pub struct M{
    frict: Handle<StandardMaterial>,
    slip: Handle<StandardMaterial>
}

// const ANGLE_LIMIT: (f32, f32) = (PI / 6., PI * 2./3.);
const ANGLE_LIMIT: (f32, f32) = (PI / 3., PI / 2.3);

#[derive(Component)]
pub struct IsPull(bool);

#[derive(Component)]
pub struct Foots(Entity, Entity);


#[derive(Event)]
pub struct IsPullChanged([Entity;2], bool);

// ---

fn startup (
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cmd: Commands
) {

    let m = M{
        frict: materials.add(Color::srgb(1., 0., 0.)),
        slip: materials.add(Color::srgb(0., 0., 1.))
    };
    cmd.insert_resource(m.clone());

    let mat = materials.add(Color::srgb(0., 0., 0.));

    let leg_dim = Vec3::new(2., 0.5, 8.);
    let foot_dim = (0.5, 5.0);
    let leg_mesh = meshes.add(Cuboid::from_size(leg_dim));
    let foot_mesh = meshes.add(Cylinder::new(foot_dim.0, foot_dim.1));
    let mut ets: [(Entity, Entity); 2] = [(Entity::PLACEHOLDER, Entity::PLACEHOLDER); 2];
    
    for z in 0..2 {
        let leg_id = cmd.spawn((
            MeshMaterial3d(mat.clone()),
            Mesh3d(leg_mesh.clone()),
            Transform::from_xyz(0., leg_dim.z * 0.5 + foot_dim.0,  2. * z as f32)
            .with_rotation(Quat::from_rotation_x(-PI * 0.5))
            ,
            MassPropertiesBundle::from_shape(&Collider::cuboid(leg_dim.x, leg_dim.y, leg_dim.z), 5.),
            RigidBody::Dynamic,
            // RigidBody::Static,
            Name::new("Leg")
            
        ))
        .with_children(|l| {
            let f_id = l.spawn((
                MeshMaterial3d(if z == 0 {m.frict.clone()}  else {m.slip.clone()}),
                Mesh3d(foot_mesh.clone()),
                Transform::from_xyz(0., 0. , leg_dim.z * -0.5).with_rotation(Quat::from_rotation_z(90.0_f32.to_radians())),
                
                Collider::cylinder(foot_dim.0, foot_dim.1),
                ColliderDensity(10.),
                Friction::new(if z == 0 {1.} else {0.}).with_combine_rule(CoefficientCombine::Min),
                Restitution::new(0.).with_combine_rule(CoefficientCombine::Min),
                GravityScale(0.5)
                // BackFoot(z == 1),
            )).id()
            ;
            ets[z].1 = f_id;
        })
        .id()
        ;
        ets[z].0 = leg_id;
    }
    cmd.spawn((
        RevoluteJoint::new(ets[0].0, ets[1].0)
        .with_aligned_axis(Vec3::X)
        .with_local_anchor_1(Vec3::Z * leg_dim.z * 0.4)
        .with_local_anchor_2(Vec3::Z * leg_dim.z * 0.4)
        .with_angle_limits(ANGLE_LIMIT.0, ANGLE_LIMIT.1)
        .with_angular_velocity_damping(10.)
        ,
        Hinge,
        IsPull(true),
        Foots(ets[0].1, ets[1].1)

    ))
    ;

}

// ---

fn pull_changed(
    tr: Trigger<IsPullChanged>, 
    mut foots_q: Query<(&mut Friction, &mut MeshMaterial3d<StandardMaterial>)>,
    mats: Res<M>,

) {
    let Ok([(mut f_f, mut m_f), (mut f_b, mut m_b)]) = foots_q.get_many_mut(tr.event().0) else {
        return;
    };
    
    if tr.event().1 {
        *f_f = Friction::new(1.).with_combine_rule(CoefficientCombine::Max);
        *f_b = Friction::new(0.).with_combine_rule(CoefficientCombine::Min);
        m_f.0 = mats.frict.clone_weak();
        m_b.0 = mats.slip.clone_weak();
    } else {
        *f_b = Friction::new(1.).with_combine_rule(CoefficientCombine::Max);
        *f_f = Friction::new(0.).with_combine_rule(CoefficientCombine::Min);
        m_b.0 = mats.frict.clone_weak();
        m_f.0 = mats.slip.clone_weak();
    }

}

// ---

fn moving(
    mut h_q: Query<(&mut RevoluteJoint, &mut IsPull, &Foots), With<Hinge>>,
    time: Res<Time>,
    mut cmd: Commands
) {
    for (mut joint, mut is_pull, f) in &mut h_q {
        let Some(angle_limit) = joint.angle_limit else {
            continue;
        };

        let point = if is_pull.0 {ANGLE_LIMIT.0} else {ANGLE_LIMIT.1};
        let rest = point - angle_limit.max;
        let delta = time.delta_secs() * 0.2;

        let diff = delta.min(rest.abs());
        if rest == diff {
            is_pull.0 = !is_pull.0;
            cmd.trigger(IsPullChanged([f.0, f.1], is_pull.0));
            continue;
        }
        let new_max =  angle_limit.max + diff * rest.signum();
        joint.angle_limit = Some(AngleLimit{max: new_max, min: angle_limit.min});
    }
}
