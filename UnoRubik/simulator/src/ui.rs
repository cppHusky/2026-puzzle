use crate::config;
use crate::facelet;
use crate::movement;
use crate::rubik;
use crate::turn;
use bevy::prelude::*;
#[derive(Debug,Clone,Component)]
struct TurnButton(turn::Turn);
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Movement{Up,Down,Left,Right}
#[derive(Debug,Clone,Component)]
struct MoveButton(Movement);
#[derive(Debug,Clone,Component)]
pub struct MoveButtonInternal(pub Movement);
#[derive(Debug,Clone,Message)]
pub struct FaceletRefreshMessage(pub facelet::Facelet);
#[derive(Debug,Clone,Message)]
pub struct MovementRefreshMessage;
pub fn setup_ui(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
    mut material_cache:ResMut<config::MaterialCache>,
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
            MeshMaterial2d(material_cache.get(config::GameColor::White,&mut materials)),
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
            MoveButton(movement.clone()),
            Mesh2d(meshes.add(Rectangle::new(30.,30.))),
            transform,
            children![(
                MoveButtonInternal(movement),
                Mesh2d(meshes.add(arrow)),
                MeshMaterial2d(material_cache.get(config::GameColor::White,&mut materials)),
                Transform::from_translation(Vec3::ZERO),
            ),(
                Mesh2d(meshes.add(Polyline2d::new(vec![
                    Vec2::new(-15.,-15.),
                    Vec2::new(15.,-15.),
                    Vec2::new(15.,15.),
                    Vec2::new(-15.,15.),
                    Vec2::new(-15.,-15.),
                ]))),
                MeshMaterial2d(material_cache.get(config::GameColor::DarkGrey,&mut materials)),
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
    mut facelet_message_writer:MessageWriter<FaceletRefreshMessage>,
    mut movement_message_writer:MessageWriter<MovementRefreshMessage>,
    mut player:ResMut<movement::Player>,
){
    if let Ok(TurnButton(turn))=buttons_query.get(trigger.entity){
        *rubik*=turn.clone();
        player.0=turn[&player.0].clone();
        if *turn==turn::U.into()||*turn==turn::u.into(){
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *turn==turn::D.into()||*turn==turn::d.into(){
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
        }
        else if *turn==turn::F.into()||*turn==turn::f.into(){
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *turn==turn::L.into()||*turn==turn::l.into(){
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
        }
        else if *turn==turn::R.into()||*turn==turn::r.into(){
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *turn==turn::_U.into()||*turn==turn::_D.into(){
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
        }
        else if *turn==turn::_L.into()||*turn==turn::_R.into(){
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            facelet_message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
        }
        movement_message_writer.write(MovementRefreshMessage);
    }
}
fn observe_move(
    trigger:On<Pointer<Click>>,
    buttons_query:Query<&MoveButton>,
    rubik:ResMut<rubik::Rubik>,
    mut player:ResMut<movement::Player>,
    mut message_writer:MessageWriter<MovementRefreshMessage>,
){
    if let Ok(MoveButton(movement))=buttons_query.get(trigger.entity){
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
        message_writer.write(MovementRefreshMessage);
    }
}
