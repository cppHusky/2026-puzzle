use bevy::prelude::*;
use crate::facelet;
#[derive(Debug,Clone,Hash,PartialEq,Eq)]
pub enum GameColor{
    Black,
    DarkGrey,
    White,
    Yellow,
    Red,
    Orange,
    Green,
    Blue,
}
impl GameColor{
    pub fn to_bevy_color(self)->Color{
        match self{
            Self::Black=>Color::from(bevy::color::palettes::css::BLACK),
            Self::DarkGrey=>Color::from(Srgba::rgb(0.3,0.3,0.3)),
            Self::White=>Color::from(bevy::color::palettes::css::WHITE),
            Self::Yellow=>Color::from(bevy::color::palettes::css::YELLOW),
            Self::Red=>Color::from(bevy::color::palettes::css::RED),
            Self::Orange=>Color::from(bevy::color::palettes::css::ORANGE),
            Self::Green=>Color::from(bevy::color::palettes::css::GREEN),
            Self::Blue=>Color::from(bevy::color::palettes::css::BLUE),
        }
    }
}
#[derive(Resource)]
pub struct MaterialCache(std::collections::HashMap<GameColor,Handle<ColorMaterial>>);
impl MaterialCache{
    pub fn new()->Self{
        Self(std::collections::HashMap::new())
    }
    pub fn get(
        &mut self,
        color:GameColor,
        materials:&mut ResMut<Assets<ColorMaterial>>,
    )->Handle<ColorMaterial>{
        self.0.entry(color.clone())
            .or_insert_with(||materials.add(ColorMaterial::from(color.to_bevy_color())))
            .clone()
    }
}
pub fn face_color(facelet:&facelet::Facelet)->GameColor{
    use facelet::Facelet::*;
    match facelet{
        Ufl|Uf|Ufr|Ul|U|Ur|Ubl|Ub|Ubr=>GameColor::White,
        Dbl|Db|Dbr|Dl|D|Dr|Dfl|Df|Dfr=>GameColor::Yellow,
        Fdl|Fd|Fdr|Fl|F|Fr|Ful|Fu|Fur=>GameColor::Red,
        Bdr|Bd|Bdl|Br|B|Bl|Bur|Bu|Bul=>GameColor::Orange,
        Ldb|Ld|Ldf|Lb|L|Lf|Lub|Lu|Luf=>GameColor::Green,
        Rdf|Rd|Rdb|Rf|R|Rb|Ruf|Ru|Rub=>GameColor::Blue,
    }
}
pub fn face_number(facelet:&facelet::Facelet)->u8{
    use facelet::Facelet::*;
    match facelet{
        Ufl=>16,
        Uf=>1,
        Ufr=>9,
        Ul=>2,
        U=>4,
        Ur=>5,
        Ubl=>0,
        Ub=>11,
        Ubr=>8,
        Dbl=>13,
        Db=>17,
        Dbr=>0,
        Dl=>15,
        D=>4,
        Dr=>3,
        Dfl=>14,
        Df=>10,
        Dfr=>6,
        Fdl=>8,
        Fd=>11,
        Fdr=>12,
        Fl=>12,
        F=>0,
        Fr=>5,
        Ful=>1,
        Fu=>2,
        Fur=>9,
        Bdr=>8,
        Bd=>17,
        Bdl=>9,
        Br=>10,
        B=>0,
        Bl=>6,
        Bur=>13,
        Bu=>15,
        Bul=>4,
        Ldb=>13,
        Ld=>5,
        Ldf=>14,
        Lb=>2,
        L=>0,
        Lf=>16,
        Lub=>7,
        Lu=>11,
        Luf=>4,
        Rdf=>4,
        Rd=>15,
        Rdb=>14,
        Rf=>7,
        R=>13,
        Rb=>11,
        Ruf=>0,
        Ru=>10,
        Rub=>3,
    }
}
