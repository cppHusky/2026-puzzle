use bevy::prelude::*;
use crate::config;
use crate::facelet;
use crate::rubik;
use crate::turn;
///It tells the actual place of the Player
#[derive(Debug,Resource)]
pub struct Player(pub facelet::Facelet);
///The mark to imply that an entity is a Player
#[derive(Debug,Component)]
pub struct PlayerMark;
///It tells whether or not the player can move to another facelet. Only when the actual facelets
///are adjacent on the front face, and the logical facelets have the same color or number.
pub fn moveable(rubik:&rubik::Rubik,af1:&facelet::Facelet,af2:&facelet::Facelet)->bool{
    let in_front_face=|actual_face:&facelet::Facelet|->bool{
        use facelet::Facelet::*;
        match actual_face{
            Fdl|Fd|Fdr|Fl|F|Fr|Ful|Fu|Fur=>true,
            _=>false,
        }
    };
    let have_same_color=|actual_face1:&facelet::Facelet,actual_face2:&facelet::Facelet|->bool{
        let (logical_face1,logical_face2)=(&rubik[actual_face1],&rubik[actual_face2]);
        config::face_color(&logical_face1)==config::face_color(&logical_face2)
    };
    let have_same_number=|actual_face1:&facelet::Facelet,actual_face2:&facelet::Facelet|->bool{
        let (logical_face1,logical_face2)=(&rubik[actual_face1],&rubik[actual_face2]);
        config::face_number(&logical_face1)==config::face_number(&logical_face2)
    };
    in_front_face(&af1)&&in_front_face(&af2)&&(have_same_color(&af1,&af2)||have_same_number(&af1,&af2))
}
pub fn move_up(from:&facelet::Facelet)->Option<facelet::Facelet>{
    match from{
        facelet::Facelet::Fdl=>Some(facelet::Facelet::Fl),
        facelet::Facelet::Fd=>Some(facelet::Facelet::F),
        facelet::Facelet::Fdr=>Some(facelet::Facelet::Fr),
        facelet::Facelet::Fl=>Some(facelet::Facelet::Ful),
        facelet::Facelet::F=>Some(facelet::Facelet::Fu),
        facelet::Facelet::Fr=>Some(facelet::Facelet::Fur),
        _=>None,
    }
}
pub fn move_down(from:&facelet::Facelet)->Option<facelet::Facelet>{
    match from{
        facelet::Facelet::Ful=>Some(facelet::Facelet::Fl),
        facelet::Facelet::Fu=>Some(facelet::Facelet::F),
        facelet::Facelet::Fur=>Some(facelet::Facelet::Fr),
        facelet::Facelet::Fl=>Some(facelet::Facelet::Fdl),
        facelet::Facelet::F=>Some(facelet::Facelet::Fd),
        facelet::Facelet::Fr=>Some(facelet::Facelet::Fdr),
        _=>None,
    }
}
pub fn move_left(from:&facelet::Facelet)->Option<facelet::Facelet>{
    match from{
        facelet::Facelet::Fd=>Some(facelet::Facelet::Fdl),
        facelet::Facelet::Fdr=>Some(facelet::Facelet::Fd),
        facelet::Facelet::F=>Some(facelet::Facelet::Fl),
        facelet::Facelet::Fr=>Some(facelet::Facelet::F),
        facelet::Facelet::Fu=>Some(facelet::Facelet::Ful),
        facelet::Facelet::Fur=>Some(facelet::Facelet::Fu),
        _=>None,
    }
}
pub fn move_right(from:&facelet::Facelet)->Option<facelet::Facelet>{
    match from{
        facelet::Facelet::Fdl=>Some(facelet::Facelet::Fd),
        facelet::Facelet::Fd=>Some(facelet::Facelet::Fdr),
        facelet::Facelet::Fl=>Some(facelet::Facelet::F),
        facelet::Facelet::F=>Some(facelet::Facelet::Fr),
        facelet::Facelet::Ful=>Some(facelet::Facelet::Fu),
        facelet::Facelet::Fu=>Some(facelet::Facelet::Fur),
        _=>None,
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_moveable(){
        use turn::*;
        let mut rubik=rubik::Rubik::new()*L*d*l*f*d*F*U*f*D*F*L*D*l*u;
        assert!(!moveable(&rubik,&facelet::Facelet::Ful,&facelet::Facelet::Fu));
        assert!(!moveable(&rubik,&facelet::Facelet::Fur,&facelet::Facelet::Fu));
        assert!(!moveable(&rubik,&facelet::Facelet::Ful,&facelet::Facelet::Fur));
        rubik*=L*_L;
        assert!(moveable(&rubik,&facelet::Facelet::Fdl,&move_right(&facelet::Facelet::Fdl).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fr,&move_left(&facelet::Facelet::Fr).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fdl,&move_up(&facelet::Facelet::Fdl).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fur,&move_down(&facelet::Facelet::Fur).unwrap()));
        rubik*=r*D;
        assert!(moveable(&rubik,&facelet::Facelet::Fdr,&move_left(&facelet::Facelet::Fdr).unwrap()));
        assert!(moveable(&rubik,&facelet::Facelet::Ful,&move_right(&facelet::Facelet::Ful).unwrap()));
        assert!(moveable(&rubik,&facelet::Facelet::Fdr,&move_up(&facelet::Facelet::Fdr).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::F,&move_down(&facelet::Facelet::F).unwrap()));
        rubik*=d*r*u*_U;
        assert!(moveable(&rubik,&facelet::Facelet::Fur,&move_down(&facelet::Facelet::Fur).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fl,&move_up(&facelet::Facelet::Fl).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fu,&move_down(&facelet::Facelet::Fu).unwrap()));
        assert!(moveable(&rubik,&facelet::Facelet::Fr,&move_left(&facelet::Facelet::Fr).unwrap()));
        rubik*=_D*U*U*d*_L;
        assert!(moveable(&rubik,&facelet::Facelet::Fdl,&move_right(&facelet::Facelet::Fdl).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fd,&move_up(&facelet::Facelet::Fd).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fur,&move_down(&facelet::Facelet::Fur).unwrap()));
        assert!(moveable(&rubik,&facelet::Facelet::F,&move_right(&facelet::Facelet::F).unwrap()));
        rubik*=_R*r*U*U*_L;
        assert!(moveable(&rubik,&facelet::Facelet::Fdr,&move_left(&facelet::Facelet::Fdr).unwrap()));
        assert!(moveable(&rubik,&facelet::Facelet::F,&move_down(&facelet::Facelet::F).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::Fu,&move_down(&facelet::Facelet::Fu).unwrap()));
        assert!(!moveable(&rubik,&facelet::Facelet::F,&move_left(&facelet::Facelet::F).unwrap()));
        assert!(moveable(&rubik,&facelet::Facelet::F,&move_right(&facelet::Facelet::F).unwrap()));
    }
}
