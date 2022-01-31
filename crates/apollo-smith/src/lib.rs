// Implement DocumentBuilder
// Implement DocumentBuilderAllocations
// Transform every function in grammar to a method of impl DocumentBuilder

// fn f64_const(
//     u: &mut Unstructured,
//     _: &Module,
//     builder: &mut CodeBuilder,
// ) -> Result<Instruction> {
//     let x = u.arbitrary::<f64>()?;
//     builder.push_operands(&[ValType::F64]);
//     Ok(Instruction::F64Const(x))
// }

use std::collections::HashSet;

use apollo_encoder::Schema;
use arbitrary::Unstructured;
use interface::InterfaceTypeDef;
use object::ObjectTypeDef;
use rand::{thread_rng, Rng};

pub(crate) mod argument;
pub(crate) mod description;
pub(crate) mod document;
pub(crate) mod field;
// pub(crate) mod grammar;
pub(crate) mod directive;
pub(crate) mod interface;
pub(crate) mod name;
pub(crate) mod object;
pub(crate) mod ty;
pub(crate) mod value;

pub struct DocumentBuilder<'a> {
    pub(crate) u: Unstructured<'a>,
    pub(crate) object_type_defs: Vec<ObjectTypeDef>,
    pub(crate) interface_type_defs: Vec<InterfaceTypeDef>,
    pub(crate) type_names: HashSet<String>,
    pub(crate) nested_level: u8,
    pub(crate) field_names: HashSet<String>,
}

impl<'a> DocumentBuilder<'a> {
    pub fn new(btes: &'a [u8]) -> Self {
        let mut builder = Self {
            u: Unstructured::new(btes),
            object_type_defs: Vec::new(),
            interface_type_defs: Vec::new(),
            type_names: HashSet::new(),
            nested_level: 0,
            field_names: HashSet::new(),
        };
        // TODO: use arbitrary
        let mut rng = thread_rng();
        for _ in 0..rng.gen_range(1..50) {
            let interface_type_def = builder.interface_type_definition();
            builder.interface_type_defs.push(interface_type_def);
        }

        for _ in 0..rng.gen_range(1..50) {
            let object_type_def = builder.object_type_definition();
            builder.object_type_defs.push(object_type_def);
        }

        builder
    }

    pub fn finish(self) -> Schema {
        let mut schema = Schema::new();
        self.object_type_defs
            .into_iter()
            .for_each(|obj| schema.object(obj.into()));
        self.interface_type_defs
            .into_iter()
            .for_each(|itf| schema.interface(itf.into()));

        schema
    }
}
