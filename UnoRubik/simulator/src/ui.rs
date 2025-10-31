use crate::facelet;
use crate::movement;
use crate::rubik;
use crate::turn;
use bevy::prelude::*;
#[derive(Debug,Clone,Component)]
struct TurnButton(turn::Turn);
#[derive(Debug,Clone)]
enum Movement{Up,Down,Left,Right}
#[derive(Debug,Clone,Component)]
struct MoveButton(Movement);
#[derive(Debug,Clone,Message)]
pub struct FaceletRefreshMessage(pub facelet::Facelet);
pub fn setup_ui(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
){
    let mut spawn_ui_turn=|turn:turn::Turn,transform:Transform|{
        let arrow=Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleList,
            bevy::asset::RenderAssetUsages::default(),
        ).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [-10.,-25.,0.],
                [-10.,5.,0.],
                [10.,-25.,0.],
                [10.,5.,0.],
                [-10.,5.,0.],
                [10.,-25.,0.],
                [-18.,5.,0.],
                [18.,5.,0.],
                [0.,25.,0.],
            ],
        );
        commands.spawn((
            TurnButton(turn),
            Mesh2d(meshes.add(arrow)),
            MeshMaterial2d(materials.add(Color::WHITE)),
            transform,
        )).observe(observe_turn);
    };
    spawn_ui_turn(turn::l.into(),Transform::from_xyz(-100.,200.,0.));
    spawn_ui_turn(turn::_R.into(),Transform::from_xyz(0.,200.,0.));
    spawn_ui_turn(turn::R.into(),Transform::from_xyz(100.,200.,0.));
    spawn_ui_turn(turn::L.into(),Transform::from_xyz(-100.,-200.,0.).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));
    spawn_ui_turn(turn::_L.into(),Transform::from_xyz(0.,-200.,0.).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));
    spawn_ui_turn(turn::r.into(),Transform::from_xyz(100.,-200.,0.).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));
    spawn_ui_turn(turn::d.into(),Transform::from_xyz(-200.,-100.,0.).with_rotation(Quat::from_rotation_z(0.5*std::f32::consts::PI)));
    spawn_ui_turn(turn::_U.into(),Transform::from_xyz(-200.,0.,0.,).with_rotation(Quat::from_rotation_z(0.5*std::f32::consts::PI)));
    spawn_ui_turn(turn::U.into(),Transform::from_xyz(-200.,100.,0.,).with_rotation(Quat::from_rotation_z(0.5*std::f32::consts::PI)));
    spawn_ui_turn(turn::D.into(),Transform::from_xyz(200.,-100.,0.).with_rotation(Quat::from_rotation_z(1.5*std::f32::consts::PI)));
    spawn_ui_turn(turn::_D.into(),Transform::from_xyz(200.,0.,0.,).with_rotation(Quat::from_rotation_z(1.5*std::f32::consts::PI)));
    spawn_ui_turn(turn::u.into(),Transform::from_xyz(200.,100.,0.,).with_rotation(Quat::from_rotation_z(1.5*std::f32::consts::PI)));
    let mut spawn_ui_movement=|movement:Movement,transform:Transform|{
        let arrow=Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleList,
            bevy::asset::RenderAssetUsages::default(),
        ).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [0.,0.,0.],
                [0.,10.,0.],
                [10.,-10.,0.],
                [0.,0.,0.],
                [0.,10.,0.],
                [-10.,-10.,0.],
            ],
        );
        commands.spawn((
            MoveButton(movement),
            Mesh2d(meshes.add(Rectangle::new(30.,30.))),
            transform,
            children![(
                Mesh2d(meshes.add(arrow)),
                MeshMaterial2d(materials.add(Color::from(Srgba::rgb(0.9,0.9,0.9)))),
                Transform::from_translation(Vec3::ZERO),
            ),(
                Mesh2d(meshes.add(Polyline2d::new(vec![
                    Vec2::new(-15.,-15.),
                    Vec2::new(15.,-15.),
                    Vec2::new(15.,15.),
                    Vec2::new(-15.,15.),
                    Vec2::new(-15.,-15.),
                ]))),
                MeshMaterial2d(materials.add(Color::from(Srgba::rgb(0.3,0.3,0.3)))),
                Transform::from_xyz(0.,0.,-1.),
            )],
        )).observe(observe_move);
    };
    spawn_ui_movement(Movement::Up,Transform::from_xyz(300.,-160.,0.));
    spawn_ui_movement(Movement::Down,Transform::from_xyz(300.,-200.,0.).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));
    spawn_ui_movement(Movement::Left,Transform::from_xyz(260.,-200.,0.).with_rotation(Quat::from_rotation_z(0.5*std::f32::consts::PI)));
    spawn_ui_movement(Movement::Right,Transform::from_xyz(340.,-200.,0.).with_rotation(Quat::from_rotation_z(1.5*std::f32::consts::PI)));
}
fn observe_turn(
    trigger:On<Pointer<Click>>,
    buttons_query:Query<&TurnButton>,
    mut rubik:ResMut<rubik::Rubik>,
    mut message_writer:MessageWriter<FaceletRefreshMessage>,
    mut player:ResMut<movement::Player>,
){
    if let Ok(TurnButton(turn))=buttons_query.get(trigger.entity){
        *rubik*=turn.clone();
        player.0=turn[&player.0].clone();
        if *turn==turn::U.into()||*turn==turn::u.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *turn==turn::D.into()||*turn==turn::d.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
        }
        else if *turn==turn::F.into()||*turn==turn::f.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *turn==turn::L.into()||*turn==turn::l.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
        }
        else if *turn==turn::R.into()||*turn==turn::r.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *turn==turn::_U.into()||*turn==turn::_D.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
        }
        else if *turn==turn::_L.into()||*turn==turn::_R.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
        }
    }
}
fn observe_move(
    trigger:On<Pointer<Click>>,
    buttons_query:Query<&MoveButton>,
    mut rubik:ResMut<rubik::Rubik>,
    mut player:ResMut<movement::Player>,
){
    if let Ok(MoveButton(movement))=buttons_query.get(trigger.entity){
        eprintln!("Move {:?}",movement);
        let new_facelet=match movement{
            Movement::Up=>movement::move_up(&player.0),
            Movement::Down=>movement::move_down(&player.0),
            Movement::Left=>movement::move_left(&player.0),
            Movement::Right=>movement::move_right(&player.0),
        };
        if let Some(new_facelet)=new_facelet{
            if movement::moveable(&rubik,&player.0,&new_facelet){
                player.0=new_facelet;
            }
        }
    }
}
