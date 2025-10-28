pub const FACELET_SIZE:usize=54;
///This enum represents all faces in the rubik.
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Facelet{//Order of characters:U,D,F,B,L,R
    Ufl,Uf,Ufr,Ul,U,Ur,Ubl,Ub,Ubr,
    Dbl,Db,Dbr,Dl,D,Dr,Dfl,Df,Dfr,
    Fdl,Fd,Fdr,Fl,F,Fr,Ful,Fu,Fur,
    Bdr,Bd,Bdl,Br,B,Bl,Bur,Bu,Bul,
    Ldb,Ld,Ldf,Lb,L,Lf,Lub,Lu,Luf,
    Rdf,Rd,Rdb,Rf,R,Rb,Ruf,Ru,Rub,
}
pub const ALL_FACELETS:[Facelet;FACELET_SIZE]=[
    Facelet::Ufl,Facelet::Uf,Facelet::Ufr,Facelet::Ul,Facelet::U,Facelet::Ur,Facelet::Ubl,Facelet::Ub,Facelet::Ubr,
    Facelet::Dbl,Facelet::Db,Facelet::Dbr,Facelet::Dl,Facelet::D,Facelet::Dr,Facelet::Dfl,Facelet::Df,Facelet::Dfr,
    Facelet::Fdl,Facelet::Fd,Facelet::Fdr,Facelet::Fl,Facelet::F,Facelet::Fr,Facelet::Ful,Facelet::Fu,Facelet::Fur,
    Facelet::Bdr,Facelet::Bd,Facelet::Bdl,Facelet::Br,Facelet::B,Facelet::Bl,Facelet::Bur,Facelet::Bu,Facelet::Bul,
    Facelet::Ldb,Facelet::Ld,Facelet::Ldf,Facelet::Lb,Facelet::L,Facelet::Lf,Facelet::Lub,Facelet::Lu,Facelet::Luf,
    Facelet::Rdf,Facelet::Rd,Facelet::Rdb,Facelet::Rf,Facelet::R,Facelet::Rb,Facelet::Ruf,Facelet::Ru,Facelet::Rub,
];
impl std::fmt::Display for Facelet{
    fn fmt(&self,f:&mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"{:?}",self)
    }
}
