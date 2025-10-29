use crate::facelet;
use crate::rubik;
use crate::movement;
use bevy::prelude::*;
#[derive(Debug,Clone,Component)]
struct UiButton(movement::Movement);
#[derive(Debug,Clone,Message)]
pub struct FaceletRefreshMessage(pub facelet::Facelet);
pub fn setup_ui(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
){
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
    let mut spawn_ui_arrow=|mov:movement::Movement,transform:Transform|{
        commands.spawn((
            UiButton(mov),
            Mesh2d(meshes.add(arrow.clone())),
            MeshMaterial2d(materials.add(Color::WHITE)),
            transform,
        )).observe(observe_movement);
    };
    spawn_ui_arrow(movement::l.into(),Transform::from_xyz(-100.,200.,0.));
    spawn_ui_arrow(movement::_R.into(),Transform::from_xyz(0.,200.,0.));
    spawn_ui_arrow(movement::R.into(),Transform::from_xyz(100.,200.,0.));
    spawn_ui_arrow(movement::L.into(),Transform::from_xyz(-100.,-200.,0.).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));
    spawn_ui_arrow(movement::_L.into(),Transform::from_xyz(0.,-200.,0.).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));
    spawn_ui_arrow(movement::r.into(),Transform::from_xyz(100.,-200.,0.).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));
    spawn_ui_arrow(movement::d.into(),Transform::from_xyz(-200.,-100.,0.).with_rotation(Quat::from_rotation_z(0.5*std::f32::consts::PI)));
    spawn_ui_arrow(movement::_U.into(),Transform::from_xyz(-200.,0.,0.,).with_rotation(Quat::from_rotation_z(0.5*std::f32::consts::PI)));
    spawn_ui_arrow(movement::U.into(),Transform::from_xyz(-200.,100.,0.,).with_rotation(Quat::from_rotation_z(0.5*std::f32::consts::PI)));
    spawn_ui_arrow(movement::D.into(),Transform::from_xyz(200.,-100.,0.).with_rotation(Quat::from_rotation_z(1.5*std::f32::consts::PI)));
    spawn_ui_arrow(movement::_D.into(),Transform::from_xyz(200.,0.,0.,).with_rotation(Quat::from_rotation_z(1.5*std::f32::consts::PI)));
    spawn_ui_arrow(movement::u.into(),Transform::from_xyz(200.,100.,0.,).with_rotation(Quat::from_rotation_z(1.5*std::f32::consts::PI)));
}
fn observe_movement(
    trigger:On<Pointer<Click>>,
    buttons_query:Query<&UiButton>,
    mut rubik:ResMut<rubik::Rubik>,
    mut message_writer:MessageWriter<FaceletRefreshMessage>,
){
    if let Ok(UiButton(mov))=buttons_query.get(trigger.entity){
        *rubik*=mov.clone();
        if *mov==movement::U.into()||*mov==movement::u.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *mov==movement::D.into()||*mov==movement::d.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
        }
        else if *mov==movement::F.into()||*mov==movement::f.into(){
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
        else if *mov==movement::L.into()||*mov==movement::l.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Ful));
        }
        else if *mov==movement::R.into()||*mov==movement::r.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fdr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fur));
        }
        else if *mov==movement::_U.into()||*mov==movement::_D.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fr));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fl));
        }
        else if *mov==movement::_L.into()||*mov==movement::_R.into(){
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fd));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::F));
            message_writer.write(FaceletRefreshMessage(facelet::Facelet::Fu));
        }
    }
}
