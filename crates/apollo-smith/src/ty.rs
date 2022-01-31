use apollo_encoder::Type_;
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};

use crate::{name::Name, DocumentBuilder};

static BUILTIN_SCALAR_NAMES: Lazy<[Ty; 5]> = Lazy::new(|| {
    [
        Ty::Named(Name::new(String::from("Int"))),
        Ty::Named(Name::new(String::from("Float"))),
        Ty::Named(Name::new(String::from("String"))),
        Ty::Named(Name::new(String::from("Boolean"))),
        Ty::Named(Name::new(String::from("ID"))),
    ]
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Named(Name),
    List(Box<Ty>),
    NonNull(Box<Ty>),
}

impl From<Ty> for Type_ {
    fn from(val: Ty) -> Self {
        match val {
            Ty::Named(name) => Type_::NamedType { name: name.into() },
            Ty::List(ty) => Type_::List {
                ty: Box::new((*ty).into()),
            },
            Ty::NonNull(ty) => Type_::NonNull {
                ty: Box::new((*ty).into()),
            },
        }
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn ty(&mut self) -> Ty {
        self.generate_ty(true)
    }

    pub fn choose_ty(&mut self, existing_types: &[Ty]) -> Ty {
        self._choose_ty(existing_types, true)
    }

    fn _choose_ty(&mut self, existing_types: &[Ty], is_nullable: bool) -> Ty {
        // Check the behavior of arbitrary
        // let arbitrary_type_kind = self
        //     .u
        //     .arbitrary::<usize>()
        //     .expect("cannot generate int in range")
        //     % 3;
        // TODO do not use rand
        let mut rng = thread_rng();
        let arbitrary_type_kind = rng.gen::<usize>() % 3;

        match arbitrary_type_kind {
            // Named type
            0 => {
                let used_type_names: Vec<&Ty> = existing_types
                    .iter()
                    .chain(BUILTIN_SCALAR_NAMES.iter())
                    .collect();

                // let arbitrary_index = self
                //     .u
                //     .arbitrary::<usize>()
                //     .expect("cannot generate int in range")
                //     % used_type_names.len();
                // TODO use arbitrary
                let arbitrary_index = rng.gen_range(0..used_type_names.len());

                used_type_names[arbitrary_index].clone()
            }
            // List type
            1 => Ty::List(Box::new(self._choose_ty(existing_types, true))),
            // Non Null type
            2 => {
                if is_nullable {
                    Ty::NonNull(Box::new(self._choose_ty(existing_types, false)))
                } else {
                    self._choose_ty(existing_types, is_nullable)
                }
            }
            _ => unreachable!(),
        }
    }

    // Private method
    fn generate_ty(&mut self, is_nullable: bool) -> Ty {
        let mut rng = thread_rng();
        let arbitrary_type_kind = rng.gen::<usize>() % 3;
        println!("------- {}", rng.gen::<usize>());
        if arbitrary_type_kind > 0 {
            panic!("FINALLY");
        }

        match arbitrary_type_kind {
            // Named type
            0 => Ty::Named(self.name()),
            // List type
            1 => Ty::List(Box::new(self.generate_ty(true))),
            // Non Null type
            2 => {
                if is_nullable {
                    Ty::NonNull(Box::new(self.generate_ty(false)))
                } else {
                    self.generate_ty(is_nullable)
                }
            }
            _ => unreachable!(),
        }
    }
}
