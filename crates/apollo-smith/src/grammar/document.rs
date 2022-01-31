use std::collections::HashSet;

use apollo_encoder::Schema;
use arbitrary::Unstructured;

use crate::{grammar::object, Node, SchemaBuilder, SchemaBuilderAllocations};

/// See: https://spec.graphql.org/October2021/#Document
///
/// *Document*
///     Definition*
pub fn document(u: &mut Unstructured<'_>) -> Schema {
    // let doc = p.start_node(SyntaxKind::DOCUMENT);
    // Creating a schema
    // Generate a TokenKind::StringValue | TokenKind::Name | TokenKind::LCurly
    // Then generate a definition
    let mut allocs = SchemaBuilderAllocations {
        options: Vec::new(),
    };
    let mut builder = SchemaBuilder {
        objects: Vec::new(),
        type_names: HashSet::new(),
        field_names: HashSet::new(),
        allocs: &mut allocs,
        nested_level: 0,
    };
    loop {
        // Generate at least 1 element
        select_definition(u, &mut builder);

        let keep_going = u.arbitrary().unwrap_or(false);
        if !keep_going {
            break;
        }
    }

    let mut new_schema = Schema::new();
    for obj in builder.objects {
        new_schema.object(obj);
    }

    new_schema

    // while let Some(node) = p.peek() {
    //     match node {
    //         TokenKind::StringValue => {
    //             let def = p.peek_data_n(2).unwrap();
    //             select_definition(def, p);
    //         }
    //         TokenKind::Name => {
    //             let def = p.peek_data().unwrap();
    //             select_definition(def, p);
    //         }
    //         TokenKind::LCurly => {
    //             let def = p.peek_data().unwrap();
    //             select_definition(def, p);
    //         }
    //         _ => break,
    //     }
    // }

    // doc.finish_node();

    // Finishing node/document
}

fn select_definition(u: &mut Unstructured, builder: &mut SchemaBuilder) {
    let new_object = object::object_type_definition(u, builder);
    builder
        .objects
        .push(new_object.try_into_object_def().unwrap());
    // match def.as_str() {
    //     // "directive" => directive::directive_definition(p),
    //     // "enum" => enum_::enum_type_definition(p),
    //     // "extend" => extensions::extensions(p),
    //     // "fragment" => fragment::fragment_definition(p),
    //     // "input" => input::input_object_type_definition(p),
    //     // "interface" => interface::interface_type_definition(p),
    //     "type" => object::object_type_definition(p),
    //     // "query" | "mutation" | "subscription" | "{" => operation::operation_definition(p),
    //     // "scalar" => scalar::scalar_type_definition(p),
    //     // "schema" => schema::schema_definition(p),
    //     // "union" => union_::union_type_definition(p),
    //     _ => p.err_and_pop("expected definition"),
    // }
}

pub(crate) fn is_definition(def: String) -> bool {
    matches!(
        def.as_str(),
        "directive"
            | "enum"
            | "extend"
            | "fragment"
            | "input"
            | "interface"
            | "type"
            | "query"
            | "mutation"
            | "subscription"
            | "{"
            | "scalar"
            | "schema"
            | "union"
    )
}

// #[cfg(test)]
// mod test {
//     use crate::{ast, Parser};

//     #[test]
//     fn it_creates_error_for_invalid_definition_and_has_nodes_for_valid_definition() {
//         let schema = r#"
// uasdf21230jkdw

// {
//     pet
//     faveSnack
// }
//         "#;
//         let parser = Parser::new(schema);

//         let ast = parser.parse();

//         assert_eq!(ast.errors().len(), 1);
//         assert_eq!(ast.document().definitions().into_iter().count(), 1);
//     }

//     #[test]
//     fn it_creates_an_error_for_a_document_with_only_an_invalid_definition() {
//         let schema = r#"dtzt7777777777t7777777777z7"#;
//         let parser = Parser::new(schema);

//         let ast = parser.parse();
//         assert_eq!(ast.errors().len(), 1);

//         let doc = ast.document();
//         assert!(doc.definitions().into_iter().next().is_none());
//     }

//     #[test]
//     fn it_accesses_definition_names() {
//         let schema = r#"
// directive @tag(name: String!) repeatable on FIELD_DEFINITION

// type ProductVariation {
//   id: ID!
// }
// scalar UUID @specifiedBy(url: "https://tools.ietf.org/html/rfc4122")

// union SearchResult = Photo | Person

// extend type Query {
//   allProducts: [Product]
//   product(id: ID!): Product
// }
//         "#;
//         let parser = crate::Parser::new(schema);
//         let ast = parser.parse();

//         assert!(ast.errors.is_empty());
//         let document = ast.document();
//         for definition in document.definitions() {
//             match definition {
//                 ast::Definition::DirectiveDefinition(directive) => {
//                     assert_eq!(
//                         directive
//                             .name()
//                             .expect("Cannot get directive name.")
//                             .text()
//                             .as_ref(),
//                         "tag"
//                     )
//                 }
//                 ast::Definition::ObjectTypeDefinition(object_type) => {
//                     assert_eq!(
//                         object_type
//                             .name()
//                             .expect("Cannot get object type definition name.")
//                             .text()
//                             .as_ref(),
//                         "ProductVariation"
//                     )
//                 }
//                 ast::Definition::UnionTypeDefinition(union_type) => {
//                     assert_eq!(
//                         union_type
//                             .name()
//                             .expect("Cannot get union type definition name.")
//                             .text()
//                             .as_ref(),
//                         "SearchResult"
//                     )
//                 }
//                 ast::Definition::ScalarTypeDefinition(scalar_type) => {
//                     assert_eq!(
//                         scalar_type
//                             .name()
//                             .expect("Cannot get scalar type definition name.")
//                             .text()
//                             .as_ref(),
//                         "UUID"
//                     )
//                 }
//                 ast::Definition::ObjectTypeExtension(object_type) => {
//                     assert_eq!(
//                         object_type
//                             .name()
//                             .expect("Cannot get object type extension name.")
//                             .text()
//                             .as_ref(),
//                         "Query"
//                     )
//                 }
//                 _ => unimplemented!(),
//             }
//         }
//     }

//     #[test]
//     fn core_schema() {
//         let schema = r#"
// schema @core(feature: "https://specs.apollo.dev/join/v0.1") {
//   query: Query
//   mutation: Mutation
// }

// enum join__Graph {
//   ACCOUNTS @join__graph(name: "accounts")
// }
//         "#;
//         let parser = crate::Parser::new(schema);
//         let ast = parser.parse();

//         assert!(ast.errors.is_empty());

//         let document = ast.document();
//         for definition in document.definitions() {
//             if let ast::Definition::EnumTypeDefinition(enum_type) = definition {
//                 let enum_name = enum_type
//                     .name()
//                     .expect("Could not get Enum Type Definition's Name");

//                 assert_eq!("join__Graph", enum_name.text().as_ref());

//                 if enum_name.text().as_ref() == "join__Graph" {
//                     if let Some(enums) = enum_type.enum_values_definition() {
//                         for enum_kind in enums.enum_value_definitions() {
//                             assert_eq!(
//                                 "ACCOUNTS",
//                                 enum_kind
//                                     .enum_value()
//                                     .unwrap()
//                                     .name()
//                                     .unwrap()
//                                     .text()
//                                     .as_ref()
//                             );
//                             check_directive_arguments(enum_kind);
//                         }
//                     }
//                 }
//             }
//         }

//         fn check_directive_arguments(enum_kind: ast::EnumValueDefinition) {
//             if let Some(directives) = enum_kind.directives() {
//                 for directive in directives.directives() {
//                     if directive
//                         .name()
//                         .and_then(|n| n.ident_token())
//                         .as_ref()
//                         .map(|id| id.text())
//                         == Some("join__graph")
//                     {
//                         for argument in directive.arguments().unwrap().arguments() {
//                             if let ast::Value::StringValue(val) =
//                                 argument.value().expect("Cannot get argument value.")
//                             {
//                                 let val: String = val.into();
//                                 assert_eq!("accounts".to_string(), val);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
