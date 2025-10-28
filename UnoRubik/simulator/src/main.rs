mod config;
mod facelet;
mod movement;
mod rubik;
mod ui;
use bevy::prelude::*;
#[derive(Debug,Clone,Resource)]
struct Player(Vec3);
fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(rubik::Rubik::new())
        .add_message::<ui::FaceletRefreshMessage>()
        .add_systems(Startup,(
            setup_camera,
            ui::setup_ui,
            setup_cube,
        ))
        .add_systems(Update,(
            update_cube_on_click,
        ))
        .run();
}
fn setup_camera(mut commands:Commands){
    commands.spawn(Camera2d);
}
fn setup_cube(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
    rubik:ResMut<rubik::Rubik>,
){
    let mut spawn_facelet=|facelet:facelet::Facelet,x:f32,y:f32|{
        let actual_facelet=&rubik[&facelet];
        commands.spawn((
            rubik::RubikFace(actual_facelet.clone()),
            Mesh2d(meshes.add(Rectangle::new(99.,99.))),
            MeshMaterial2d(materials.add(config::face_color(&actual_facelet))),
            Transform::from_xyz(x,y,0.),
            children![(
                rubik::RubikFaceText(actual_facelet.clone()),
                Text2d::new(config::face_number(actual_facelet).to_string()),
                TextLayout::new(Justify::Center,LineBreak::NoWrap),
                TextFont{
                    font_size:50.,
                    ..default()
                },
                TextColor::BLACK,
                Transform::from_xyz(0.,0.,0.),
            )],
        ));
    };
    spawn_facelet(facelet::Facelet::Fdl,-100.,-100.);
    spawn_facelet(facelet::Facelet::Fd,0.,-100.);
    spawn_facelet(facelet::Facelet::Fdr,100.,-100.);
    spawn_facelet(facelet::Facelet::Fl,-100.,0.);
    spawn_facelet(facelet::Facelet::F,0.,0.);
    spawn_facelet(facelet::Facelet::Fr,100.,0.);
    spawn_facelet(facelet::Facelet::Ful,-100.,100.);
    spawn_facelet(facelet::Facelet::Fu,0.,100.);
    spawn_facelet(facelet::Facelet::Fur,100.,100.);
}
fn update_cube_on_click(
    mut materials:ResMut<Assets<ColorMaterial>>,
    rubik:ResMut<rubik::Rubik>,
    mut face_query:Query<(&rubik::RubikFace,&mut MeshMaterial2d<ColorMaterial>)>,
    mut face_text_query:Query<(&rubik::RubikFaceText,&mut Text2d)>,
    mut message_reader:MessageReader<ui::FaceletRefreshMessage>,
){
    message_reader.read().for_each(|message|{
        if let Some((_,mut material))=face_query.iter_mut().filter(|(rubik_face,_)|{
            rubik_face.0==message.0
        }).next(){
            let actual_facelet=&rubik[&message.0];
            *material=MeshMaterial2d(materials.add(config::face_color(&actual_facelet)));
        }
        if let Some((_,mut text))=face_text_query.iter_mut().filter(|(rubik_face_text,_)|{
            rubik_face_text.0==message.0
        }).next(){
            let actual_facelet=&rubik[&message.0];
            *text=Text2d::new(config::face_number(actual_facelet).to_string());
        }
    });
}
