use crate::utils::Loc;

#[derive(Debug)]
pub enum Lit {
    Int32(Int32Lit),
}

impl Lit {
    // pub fn as_int32(&self) -> Option<&Int32Lit> {
    //     match self {
    //         Lit::Int32(lit) => Some(lit),
    //         _ => None,
    //     }
    // }

    pub fn loc(&self) -> Loc {
        match self {
            Lit::Int32(lit) => lit.loc.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Int32Lit {
    pub loc: Loc,
    pub num: String,
}
