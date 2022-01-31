#![allow(clippy::needless_return)]

use apollo_encoder::{Field, InputValue};
use arbitrary::Unstructured;

use crate::{
    grammar::{description, name, ty},
    Node, SchemaBuilder,
};

use super::{name::limited_string, ty::ty};

/// See: https://spec.graphql.org/October2021/#Field
///
/// *Field*:
///     Alias? Name Arguments? Directives? SelectionSet?
pub(crate) fn field(u: &mut Unstructured, builder: &mut SchemaBuilder) -> Node {
    // =================================
    let field_name = {
        let mut fname = name::name(u, builder).try_into_name().unwrap();
        if builder.field_names.contains(&fname) {
            fname.push_str(&format!("{}", builder.field_names.len()));
        }
        fname
    };
    builder.field_names.insert(field_name.clone());

    let field_type = ty::ty(u, builder).try_into_type().unwrap();
    let new_field = Field::new(field_name, field_type);

    // TODO add possibility of having arguments/directives/selections

    Node::Field(new_field)
    // let guard = p.start_node(SyntaxKind::FIELD);

    // if let Some(TokenKind::Name) = p.peek() {
    //     if let Some(T![:]) = p.peek_n(2) {
    //         name::alias(p)
    //     }
    //     name::name(p)
    // } else {
    //     p.err("expected a Name");
    // }

    // if let Some(T!['(']) = p.peek() {
    //     argument::arguments(p);
    // }

    // if let Some(T![@]) = p.peek() {
    //     directive::directives(p);
    // }

    // if let Some(T!['{']) = p.peek() {
    //     selection::selection_set(p);
    // }

    // match p.peek() {
    //     Some(TokenKind::Name) => {
    //         guard.finish_node();
    //         field(p)
    //     }

    //     Some(T!['}']) => {
    //         guard.finish_node();
    //     }
    //     _ => guard.finish_node(),
    // }
}

// /// See: https://spec.graphql.org/October2021/#FieldsDefinition
// ///
// /// *FieldsDefinition*:
// ///     **{** FieldDefinition* **}**
// pub(crate) fn fields_definition(p: &mut Parser) {
//     let _g = p.start_node(SyntaxKind::FIELDS_DEFINITION);
//     p.bump(S!['{']);
//     field_definition(p);
//     p.expect(T!['}'], S!['}']);
// }

/// See: https://spec.graphql.org/October2021/#FieldDefinition
///
/// *FieldDefinition*:
///     Description? Name ArgumentsDefinition? **:** Type Directives?
pub(crate) fn field_definition(u: &mut Unstructured, builder: &mut SchemaBuilder) -> Node {
    // =================================
    let field_name = {
        let mut fname = name::name(u, builder).try_into_name().unwrap();
        if builder.field_names.contains(&fname) {
            fname.push_str(&format!("{}", builder.field_names.len()));
        }
        fname
    };
    builder.field_names.insert(field_name.clone());

    let field_type = ty::ty(u, builder).try_into_type().unwrap();
    let mut new_field = Field::new(field_name, field_type);

    // Description
    if u.arbitrary().unwrap_or(false) {
        new_field.description(Some(
            description::description(u, builder)
                .try_into_description()
                .unwrap(),
        ));
    }

    // arguments
    if u.arbitrary().unwrap_or(false) {
        let arb_num = u.arbitrary::<usize>().unwrap();
        for _ in 0..=arb_num {
            let input_value = InputValue::new(
                limited_string(20, u),
                ty::ty(u, builder).try_into_type().unwrap(),
            );
            new_field.arg(input_value);
        }
    }

    // directives
    // TODO not already implemented in apollo_encoder
    if u.arbitrary().unwrap_or(false) {
        for _ in 0..=u.arbitrary::<usize>().unwrap() {}
    }

    Node::Field(new_field)
    // if let Some(TokenKind::Name | TokenKind::StringValue) = p.peek() {
    //     let guard = p.start_node(SyntaxKind::FIELD_DEFINITION);

    //     if let Some(TokenKind::StringValue) = p.peek() {
    //         description::description(p);
    //     }

    //     name::name(p);

    //     if let Some(T!['(']) = p.peek() {
    //         argument::arguments_definition(p);
    //     }

    //     if let Some(T![:]) = p.peek() {
    //         p.bump(S![:]);
    //         match p.peek() {
    //             Some(TokenKind::Name) | Some(T!['[']) => {
    //                 ty::ty(p);
    //                 if let Some(T![@]) = p.peek() {
    //                     directive::directives(p);
    //                 }
    //                 if p.peek().is_some() {
    //                     guard.finish_node();
    //                     return field_definition(p);
    //                 }
    //             }
    //             _ => {
    //                 p.err("expected a Type");
    //             }
    //         }
    //     } else {
    //         p.err("expected a type");
    //     }
    // }

    // if let Some(T!['}']) = p.peek() {
    //     return;
    // }
}
