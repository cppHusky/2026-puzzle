use bevy::prelude::*;
use crate::{facelet,turn};
#[derive(Debug,Clone,Component)]
pub struct RubikFace(pub facelet::Facelet);
#[derive(Debug,Clone,Component)]
pub struct RubikFaceText(pub facelet::Facelet);
///This struct represents that after a series of turns, the facelet `value` stands at place
///`key`
#[derive(Debug,Default,Clone,PartialEq,Eq,Resource)]
pub struct Rubik(pub std::collections::HashMap<facelet::Facelet,facelet::Facelet>);
impl Rubik{
    #[allow(unused)]
    pub fn new()->Self{
        Self(facelet::ALL_FACELETS.iter().map(|c|(c.clone(),c.clone())).collect())
    }
    #[allow(unused)]
    pub fn from(facelets:[facelet::Facelet;facelet::FACELET_SIZE])->Self{
        Self(facelet::ALL_FACELETS.iter().zip(facelets.iter()).map(|(k,v)|(k.clone(),v.clone())).collect())
    }
    pub fn from_hashmap(map:std::collections::HashMap<facelet::Facelet,facelet::Facelet>)->Self{
        Self(map)
    }
}
impl std::ops::Index<&facelet::Facelet> for Rubik{
    type Output=facelet::Facelet;
    fn index(&self,id:&facelet::Facelet)->&Self::Output{
        &self.0[id]
    }
}
impl<T:Into<turn::Turn>> std::ops::Mul<T> for Rubik{
    type Output=Self;
    fn mul(self,mov:T)->Self::Output{
        let mov=mov.into();
        Self::from_hashmap(self.0.iter().map(|(k,v)|(mov[k].clone(),v.clone())).collect())
    }
}
impl<T:Into<turn::Turn>> std::ops::MulAssign<T> for Rubik{
    fn mul_assign(&mut self,mov:T){
        *self=std::mem::take(self)*mov;
    }
}
impl std::fmt::Display for Rubik{
    fn fmt(&self,f:&mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"{:?}",self.0)
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn flip_and_rotate(){
        let expectation=||->Rubik{
            use facelet::Facelet::*;
            Rubik::from([
                Ful,Fu,Fur,Lu,U,Ru,Bul,Bu,Bur,
                Bdl,Bd,Bdr,Ld,D,Rd,Fdl,Fd,Fdr,
                Ldf,Df,Rdf,Lf,F,Rf,Luf,Uf,Ruf,
                Rdb,Db,Ldb,Rb,B,Lb,Rub,Ub,Lub,
                Dbl,Dl,Dfl,Bl,L,Fl,Ubl,Ul,Ufl,
                Dfr,Dr,Dbr,Fr,R,Br,Ufr,Ur,Ubr,
            ])
        }();
        let reality=||->Rubik{
            use turn::{U,u,D,d,F,f,B,b,L,l,R,r};
            Rubik::new()
                *F*R*B*L*U*l*U*b*r*f*l*u*L*u
                *B*L*F*R*U*r*U*f*l*b*r*u*R*u
                *L*U*R*D*B*d*B*r*u*l*d*b*D*b
                *B*U*F*D*R*d*R*f*u*b*d*r*D*r
                *R*U*L*D*F*d*F*l*u*r*d*f*D*f
                *F*U*B*D*L*d*L*b*u*f*d*l*D*l
                *L*d*l*f*d*F*U*f*D*F*L*D*l*u
                *R*d*r*b*d*B*U*b*D*B*R*D*r*u
                *R*u*r*f*u*F*D*f*U*F*R*U*r*d
                *L*u*l*b*u*B*D*b*U*B*L*U*l*d
        }();
        assert_eq!(expectation,reality);
    }
}
