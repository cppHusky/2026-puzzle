mod config;
mod facelet;
mod movement;
mod rubik;
mod turn;
mod ui;
use bevy::prelude::*;
fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(rubik::Rubik::new())
        .insert_resource(movement::Player(facelet::Facelet::F))
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
    player:ResMut<movement::Player>,
    rubik:ResMut<rubik::Rubik>,
){
    let mut spawn_facelet=|facelet:facelet::Facelet,x:f32,y:f32|{
        let logical_facelet=&rubik[&facelet];
        commands.spawn((
            rubik::RubikFace(logical_facelet.clone()),
            Mesh2d(meshes.add(Rectangle::new(99.,99.))),
            MeshMaterial2d(materials.add(config::face_color(&logical_facelet))),
            Transform::from_xyz(x,y,0.),
            children![(
                rubik::RubikFaceText(logical_facelet.clone()),
                Text2d::new(config::face_number(logical_facelet).to_string()),
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
    let mut spawn_player=||{
        let pos:Transform=match player.0{
            facelet::Facelet::Fdl=>Transform::from_xyz(-100.,-100.,0.),
            facelet::Facelet::Fd=>Transform::from_xyz(0.,-100.,0.),
            facelet::Facelet::Fdr=>Transform::from_xyz(100.,-100.,0.),
            facelet::Facelet::Fl=>Transform::from_xyz(-100.,0.,0.),
            facelet::Facelet::F=>Transform::from_xyz(0.,0.,0.),
            facelet::Facelet::Fr=>Transform::from_xyz(100.,0.,0.),
            facelet::Facelet::Ful=>Transform::from_xyz(-100.,100.,0.),
            facelet::Facelet::Fu=>Transform::from_xyz(0.,100.,0.),
            facelet::Facelet::Fur=>Transform::from_xyz(100.,100.,0.),
            _=>{return;}
        };
        commands.spawn((
            movement::PlayerMark,
            Mesh2d(meshes.add(Annulus::new(35.,40.))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::BLACK))),
            pos,
        ));
    };
    spawn_player();
}
fn update_cube_on_click(
    mut commands:Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
    rubik:ResMut<rubik::Rubik>,
    mut face_query:Query<(&rubik::RubikFace,&mut MeshMaterial2d<ColorMaterial>)>,
    mut face_text_query:Query<(&rubik::RubikFaceText,&mut Text2d)>,
    mut message_reader:MessageReader<ui::FaceletRefreshMessage>,
    player:ResMut<movement::Player>,
    mut player_mark_query:Query<Entity,With<movement::PlayerMark>>,
){
    message_reader.read().for_each(|message|{
        let logical_facelet=&rubik[&message.0];
        if let Some((_,mut material))=face_query.iter_mut().filter(|(rubik_face,_)|{
            rubik_face.0==message.0
        }).next(){
            *material=MeshMaterial2d(materials.add(config::face_color(&logical_facelet)));
        }
        if let Some((_,mut text))=face_text_query.iter_mut().filter(|(rubik_face_text,_)|{
            rubik_face_text.0==message.0
        }).next(){
            *text=Text2d::new(config::face_number(logical_facelet).to_string());
        }
    });
    if let Some(p)=player_mark_query.iter_mut().next(){
        commands.entity(p).despawn();
    }
    let mut spawn_player=||{
        let pos:Transform=match player.0{
            facelet::Facelet::Fdl=>Transform::from_xyz(-100.,-100.,0.),
            facelet::Facelet::Fd=>Transform::from_xyz(0.,-100.,0.),
            facelet::Facelet::Fdr=>Transform::from_xyz(100.,-100.,0.),
            facelet::Facelet::Fl=>Transform::from_xyz(-100.,0.,0.),
            facelet::Facelet::F=>Transform::from_xyz(0.,0.,0.),
            facelet::Facelet::Fr=>Transform::from_xyz(100.,0.,0.),
            facelet::Facelet::Ful=>Transform::from_xyz(-100.,100.,0.),
            facelet::Facelet::Fu=>Transform::from_xyz(0.,100.,0.),
            facelet::Facelet::Fur=>Transform::from_xyz(100.,100.,0.),
            _=>{return;}
        };
        commands.spawn((
            movement::PlayerMark,
            Mesh2d(meshes.add(Annulus::new(35.,40.))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::BLACK))),
            pos,
        ));
    };
    spawn_player();
}
