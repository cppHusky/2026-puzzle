use crate::{facelet};
///This struct represents a turn operation, it can be multiplied with other turns, or
///multiplied with `Rubik`.
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Turn{
    cycle:std::collections::HashMap<facelet::Facelet,facelet::Facelet>,
}
impl Turn{
    #[allow(unused)]
    pub fn new()->Self{
        Self{
            cycle:facelet::ALL_FACELETS.iter().map(|c|(c.clone(),c.clone())).collect(),
        }
    }
    pub fn from(cycles:Vec<Vec<facelet::Facelet>>)->Self{
        let mut turn_cycle:std::collections::HashMap<facelet::Facelet,facelet::Facelet>=facelet::ALL_FACELETS.iter().map(|c|(c.clone(),c.clone())).collect();
        for mut cycle in cycles{
            if cycle.is_empty(){
                continue;
            }
            let last_key=cycle.last().unwrap().clone();
            let mut value=cycle.pop().unwrap();
            while let Some(key)=cycle.pop(){
                turn_cycle.insert(key.clone(),value);
                value=key;
            }
            turn_cycle.insert(last_key,value);
        }
        Self{
            cycle:turn_cycle,
        }
    }
    pub fn from_hashmap(cycle:std::collections::HashMap<facelet::Facelet,facelet::Facelet>)->Self{
        Self{cycle}
    }
}
impl<T:Into<Turn>> std::ops::Mul<T> for Turn{
    type Output=Self;
    fn mul(self,rhs:T)->Self::Output{
        let rhs=rhs.into();
        Self::from_hashmap(self.cycle.iter().map(|(k,v)|(k.clone(),rhs[v].clone())).collect())
    }
}
impl std::ops::Index<&facelet::Facelet> for Turn{
    type Output=facelet::Facelet;
    fn index(&self,index:&facelet::Facelet)->&Self::Output{
        &self.cycle[index]
    }
}
impl std::fmt::Display for Turn{
    fn fmt(&self,formatter:&mut std::fmt::Formatter)->std::fmt::Result{
        let mut map=self.cycle.clone();
        let mut cycles=vec![];
        while !map.is_empty(){
            let key=map.keys().next().unwrap().clone();
            let (key,value)=map.remove_entry(&key).unwrap();
            if key!=value{
                let mut value=value;
                let mut cycle=vec![value.clone()];
                while let Some(kv)=map.remove_entry(&value){
                    cycle.push(kv.1.clone());
                    value=kv.1;
                }
                cycles.push(cycle);
            }
        }
        write!(formatter,"{:?}",cycles)
    }
}
///The following structs are all constants that can be converted into Turn.
pub struct U;
impl Into<Turn> for U{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Ufl,Ubl,Ubr,Ufr],vec![Ful,Lub,Bur,Ruf],vec![Luf,Bul,Rub,Fur],
            vec![Uf,Ul,Ub,Ur],vec![Fu,Lu,Bu,Ru],
        ])
    }
}
#[allow(non_camel_case_types)]
pub struct u;
impl Into<Turn> for u{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Ufr,Ubr,Ubl,Ufl],vec![Ruf,Bur,Lub,Ful],vec![Fur,Rub,Bul,Luf],
            vec![Ur,Ub,Ul,Uf],vec![Ru,Bu,Lu,Fu],
        ])
    }
}
pub struct D;
impl Into<Turn> for D{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Dbl,Dfl,Dfr,Dbr],vec![Bdl,Ldf,Fdr,Rdb],vec![Ldb,Fdl,Rdf,Bdr],
            vec![Db,Dl,Df,Dr],vec![Bd,Ld,Fd,Rd],
        ])
    }
}
#[allow(non_camel_case_types)]
pub struct d;
impl Into<Turn> for d{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Dbr,Dfr,Dfl,Dbl],vec![Rdb,Fdr,Ldf,Bdl],vec![Bdr,Rdf,Fdl,Ldb],
            vec![Dr,Df,Dl,Db],vec![Rd,Fd,Ld,Bd],
        ])
    }
}
pub struct F;
impl Into<Turn> for F{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Fdl,Ful,Fur,Fdr],vec![Dfl,Luf,Ufr,Rdf],vec![Ldf,Ufl,Ruf,Dfr],
            vec![Fd,Fl,Fu,Fr],vec![Df,Lf,Uf,Rf],
        ])
    }
}
#[allow(non_camel_case_types)]
pub struct f;
impl Into<Turn> for f{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Fdr,Fur,Ful,Fdl],vec![Rdf,Ufr,Luf,Dfl],vec![Dfr,Ruf,Ufl,Ldf],
            vec![Fr,Fu,Fl,Fd],vec![Rf,Uf,Lf,Df],
        ])
    }
}
pub struct B;
impl Into<Turn> for B{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Bdr,Bur,Bul,Bdl],vec![Dbr,Rub,Ubl,Ldb],vec![Rdb,Ubr,Lub,Dbl],
            vec![Bd,Br,Bu,Bl],vec![Db,Rb,Ub,Lb],
        ])
    }
}
#[allow(non_camel_case_types)]
pub struct b;
impl Into<Turn> for b{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Bdl,Bul,Bur,Bdr],vec![Ldb,Ubl,Rub,Dbr],vec![Dbl,Lub,Ubr,Rdb],
            vec![Bl,Bu,Br,Bd],vec![Lb,Ub,Rb,Db],
        ])
    }
}
pub struct L;
impl Into<Turn> for L{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Ldb,Lub,Luf,Ldf],vec![Dbl,Bul,Ufl,Fdl],vec![Bdl,Ubl,Ful,Dfl],
            vec![Ld,Lb,Lu,Lf],vec![Dl,Bl,Ul,Fl],
        ])
    }
}
#[allow(non_camel_case_types)]
pub struct l;
impl Into<Turn> for l{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Ldf,Luf,Lub,Ldb],vec![Fdl,Ufl,Bul,Dbl],vec![Dfl,Ful,Ubl,Bdl],
            vec![Lf,Lu,Lb,Ld],vec![Fl,Ul,Bl,Dl],
        ])
    }
}
pub struct R;
impl Into<Turn> for R{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Rdf,Ruf,Rub,Rdb],vec![Dfr,Fur,Ubr,Bdr],vec![Fdr,Ufr,Bur,Dbr],
            vec![Rd,Rf,Ru,Rb],vec![Dr,Fr,Ur,Br],
        ])
    }
}
#[allow(non_camel_case_types)]
pub struct r;
impl Into<Turn> for r{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Rdb,Rub,Ruf,Rdf],vec![Bdr,Ubr,Fur,Dfr],vec![Dbr,Bur,Ufr,Fdr],
            vec![Rb,Ru,Rf,Rd],vec![Br,Ur,Fr,Dr],
        ])
    }
}
pub struct _U;
impl Into<Turn> for _U{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Fl,Lb,Br,Rf],vec![Lf,Bl,Rb,Fr],
            vec![F,L,B,R],
        ])
    }
}
pub struct _D;
impl Into<Turn> for _D{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Rf,Br,Lb,Fl],vec![Fr,Rb,Bl,Lf],
            vec![F,R,B,L],
        ])
    }
}
pub struct _F;
impl Into<Turn> for _F{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Ul,Ru,Dr,Ld],vec![Lu,Ur,Rd,Dl],
            vec![U,R,D,L],
        ])
    }
}
pub struct _B;
impl Into<Turn> for _B{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Ld,Dr,Ru,Ul],vec![Dl,Rd,Ur,Lu],
            vec![L,D,R,U],
        ])
    }
}
pub struct _L;
impl Into<Turn> for _L{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Uf,Fd,Db,Bu],vec![Fu,Df,Bd,Ub],
            vec![U,F,D,B],
        ])
    }
}
pub struct _R;
impl Into<Turn> for _R{
    fn into(self)->Turn{
        use crate::facelet::Facelet::*;
        Turn::from(vec![
            vec![Bu,Db,Fd,Uf],vec![Ub,Bd,Df,Fu],
            vec![B,D,F,U],
        ])
    }
}
macro_rules! impl_mul{
    ($($t:ty),*)=>{
        $(
            impl<T:Into<Turn>> std::ops::Mul<T> for $t{
                type Output=Turn;
                fn mul(self,rhs:T)->Self::Output{
                    let lhs:Turn=self.into();
                    let rhs:Turn=rhs.into();
                    lhs*rhs
                }
            }
        )*
    };
}
impl_mul!(U,u,D,d,F,f,B,b,L,l,R,r,_U,_D,_F,_B,_L,_R);
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn basic_laws(){
        assert_eq!(U*u,Turn::new());
        assert_eq!(D*d,Turn::new());
        assert_eq!(F*f,Turn::new());
        assert_eq!(B*b,Turn::new());
        assert_eq!(L*l,Turn::new());
        assert_eq!(R*r,Turn::new());
        assert_eq!(U*U*U*U,Turn::new());
        assert_eq!(D*D*D*D,Turn::new());
        assert_eq!(F*F*F*F,Turn::new());
        assert_eq!(B*B*B*B,Turn::new());
        assert_eq!(L*L*L*L,Turn::new());
        assert_eq!(R*R*R*R,Turn::new());
    }
    #[test]
    fn flip_uf_ul(){
        let expectation=Turn::from(vec![
            vec![facelet::Facelet::Uf,facelet::Facelet::Fu],
            vec![facelet::Facelet::Ul,facelet::Facelet::Lu],
        ]);
        let reality=F*R*B*L*U*l*U*b*r*f*l*u*L*u;
        assert_eq!(expectation,reality);
    }
    #[test]
    fn rotate_ufl_ufr(){
        let expectation=Turn::from(vec![
            vec![facelet::Facelet::Ufl,facelet::Facelet::Luf,facelet::Facelet::Ful],
            vec![facelet::Facelet::Ufr,facelet::Facelet::Ruf,facelet::Facelet::Fur],
        ]);
        let reality=L*d*l*f*d*F*U*f*D*F*L*D*l*u;
        assert_eq!(expectation,reality);
    }
    #[test]
    fn cycle_rf_uf_fl(){
        let expectation=Turn::from(vec![
            vec![facelet::Facelet::Uf,facelet::Facelet::Fl,facelet::Facelet::Fr],
            vec![facelet::Facelet::Fu,facelet::Facelet::Lf,facelet::Facelet::Rf],
        ]);
        let reality=r*d*l*F*F*L*D*R*U*F*F*u;
        assert_eq!(expectation,reality);
    }
    #[test]
    fn cycle_ufl_ufr_dfr(){
        let expectation=Turn::from(vec![
            vec![facelet::Facelet::Ful,facelet::Facelet::Fur,facelet::Facelet::Dfr],
            vec![facelet::Facelet::Ufl,facelet::Facelet::Ruf,facelet::Facelet::Rdf],
            vec![facelet::Facelet::Luf,facelet::Facelet::Ufr,facelet::Facelet::Fdr],
        ]);
        let reality=U*L*u*R*U*l*u*r;
        assert_eq!(expectation,reality);
    }
    #[test]
    fn swap_ufl_ufr_uf_ur(){
        let expectation=Turn::from(vec![
            vec![facelet::Facelet::Ful,facelet::Facelet::Ufr],
            vec![facelet::Facelet::Ufl,facelet::Facelet::Fur],
            vec![facelet::Facelet::Luf,facelet::Facelet::Ruf],
            vec![facelet::Facelet::Fu,facelet::Facelet::Ru],
            vec![facelet::Facelet::Uf,facelet::Facelet::Ur],
        ]);
        let reality=L*u*l*U*L*u*f*l*B*l*b*l*F*l*l;
        assert_eq!(expectation,reality);
    }
    #[test]
    fn cycle_uf_ub_db(){
        let expectation=Turn::from(vec![
            vec![facelet::Facelet::Uf,facelet::Facelet::Ub,facelet::Facelet::Db],
            vec![facelet::Facelet::Fu,facelet::Facelet::Bu,facelet::Facelet::Bd],
        ]);
        let reality=_L*U*U*_R*U*U;
        assert_eq!(expectation,reality);
    }
}
