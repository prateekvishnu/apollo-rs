use apollo_encoder::Type_;
use arbitrary::Unstructured;

use crate::{grammar::name, Node, SchemaBuilder};

static BUILTIN_SCALAR_NAMES: &[&str] = &["Int", "Float", "String", "Boolean", "ID"];

/// See: https://spec.graphql.org/October2021/#InputValueDefinition
///
/// *Type*:
///     NamedType
///     ListType
///         **[** Type **]**
///     NonNullType
///         NamedType **!**
///         ListType **!**

// NOTE(lrlna): Because Type cannot be parsed in a typical LR fashion, the
// following parsing rule does not follow the same pattern as all other parsing
// rules in this library. The parent node type is determined based on what its
// last possible NonNullType.
//
// To make this work, we first collect all types in a double ended queue, and
// unwrap them once the last possible child has been parsed. Nodes are then
// created in the processing stage of this parsing rule.
pub(crate) fn ty(u: &mut Unstructured, builder: &mut SchemaBuilder) -> Node {
    // TODO I don't know why int_in_range doesn't work
    let new_type = generate_ty(u, builder, false);

    // // Choose a name between existing and builtin scalars

    Node::Type(new_type)
    // let ty = parse(p);
    // process(ty, p);
}

fn generate_ty(u: &mut Unstructured, builder: &mut SchemaBuilder, already_non_null: bool) -> Type_ {
    builder.nested_level += 1;

    if builder.nested_level > 10 {
        return generate_named_type(u, builder);
    }
    // TODO I don't know why int_in_range doesn't work
    let new_type = match u.arbitrary::<u8>().expect("cannot generate int in range") % 3 {
        0 if !already_non_null => {
            // Generate NonNull type
            Type_::NonNull {
                ty: Box::new(generate_ty(u, builder, true)),
            }
        }
        1 => {
            // Generate named type
            generate_named_type(u, builder)
        }
        2 => {
            // Generate list type
            Type_::List {
                ty: Box::new(generate_ty(u, builder, already_non_null)),
            }
        }
        _ => generate_ty(u, builder, already_non_null),
    };

    builder.nested_level -= 1;

    // // Choose a name between existing and builtin scalars

    new_type
}

fn generate_named_type(u: &mut Unstructured, builder: &mut SchemaBuilder) -> Type_ {
    // TODO add also scalars
    let used_type_names: Vec<&str> = builder
        .type_names
        .iter()
        .map(|t| t.as_str())
        .chain(BUILTIN_SCALAR_NAMES.iter().copied())
        .collect();

    let arbitrary_index = u
        .arbitrary::<usize>()
        .expect("cannot generate int in range")
        % used_type_names.len();

    Type_::NamedType {
        name: used_type_names[arbitrary_index].to_string(),
    }
}

// #[derive(Debug)]
// enum TokenTy {
//     List {
//         nullable: Option<Token>,
//         open_token: Token,
//         close_token: Option<Token>,
//         inner: Box<TokenTy>,
//         comma: Option<Token>,
//         trailing_ws: Option<Token>,
//     },
//     Named {
//         nullable: Option<Token>,
//         token: Token,
//         comma: Option<Token>,
//         trailing_ws: Option<Token>,
//     },
// }

// fn parse(p: &mut Parser) -> TokenTy {
//     let token = p.pop();
//     let mut types = match token.kind() {
//         T!['['] => {
//             let inner = parse(p);
//             let close_token = if let Some(T![']']) = p.peek() {
//                 Some(p.pop())
//             } else {
//                 None
//             };

//             TokenTy::List {
//                 inner: Box::new(inner),
//                 open_token: token,
//                 close_token,
//                 nullable: None,
//                 comma: None,
//                 trailing_ws: None,
//             }
//         }
//         TokenKind::Name => TokenTy::Named {
//             token,
//             nullable: None,
//             comma: None,
//             trailing_ws: None,
//         },
//         // TODO(@lrlna): this should not panic
//         token => panic!("unexpected token, {:?}", token),
//     };

//     // Deal with nullable types
//     if let Some(T![!]) = p.peek() {
//         match &mut types {
//             TokenTy::List { nullable, .. } => nullable.replace(p.pop()),
//             TokenTy::Named { nullable, .. } => nullable.replace(p.pop()),
//         };
//     }

//     // deal with ignored tokens
//     if let Some(T![,]) = p.peek() {
//         match &mut types {
//             TokenTy::List { comma, .. } => comma.replace(p.pop()),
//             TokenTy::Named { comma, .. } => comma.replace(p.pop()),
//         };
//     }

//     if let Some(TokenKind::Whitespace) = p.peek() {
//         match &mut types {
//             TokenTy::List { trailing_ws, .. } => trailing_ws.replace(p.pop()),
//             TokenTy::Named { trailing_ws, .. } => trailing_ws.replace(p.pop()),
//         };
//     }

//     types
// }

// fn process(ty: TokenTy, p: &mut Parser) {
//     match ty {
//         TokenTy::List {
//             nullable,
//             open_token,
//             close_token,
//             inner,
//             comma,
//             trailing_ws,
//         } => match nullable {
//             Some(nullable_token) => {
//                 let _non_null_g = p.start_node(SyntaxKind::NON_NULL_TYPE);
//                 process_list(p, open_token, *inner, close_token);
//                 p.push_ast(S![!], nullable_token);
//                 process_ignored_tokens(comma, p, trailing_ws);
//             }
//             None => {
//                 process_list(p, open_token, *inner, close_token);
//                 process_ignored_tokens(comma, p, trailing_ws);
//             }
//         },
//         TokenTy::Named {
//             nullable,
//             token,
//             comma,
//             trailing_ws,
//         } => match nullable {
//             Some(nullable_token) => {
//                 let _non_null_g = p.start_node(SyntaxKind::NON_NULL_TYPE);
//                 process_named(p, token);

//                 p.push_ast(S![!], nullable_token);
//                 process_ignored_tokens(comma, p, trailing_ws);
//             }
//             None => {
//                 process_named(p, token);
//                 process_ignored_tokens(comma, p, trailing_ws);
//             }
//         },
//     }
// }

// fn process_ignored_tokens(comma: Option<Token>, p: &mut Parser, whitespace: Option<Token>) {
//     if let Some(comma_token) = comma {
//         p.push_ast(SyntaxKind::COMMA, comma_token);
//     }
//     if let Some(ws_token) = whitespace {
//         p.push_ast(SyntaxKind::WHITESPACE, ws_token);
//     }
// }

// fn process_list(p: &mut Parser, open_token: Token, inner: TokenTy, close_token: Option<Token>) {
//     let _list_g = p.start_node(SyntaxKind::LIST_TYPE);
//     p.push_ast(S!['['], open_token);
//     process(inner, p);
//     if let Some(close_token) = close_token {
//         p.push_ast(S![']'], close_token);
//     }
// }

// fn process_named(p: &mut Parser, token: Token) {
//     let named_g = p.start_node(SyntaxKind::NAMED_TYPE);
//     let name_g = p.start_node(SyntaxKind::NAME);
//     name::validate_name(token.data().to_string(), p);
//     p.push_ast(SyntaxKind::IDENT, token);
//     name_g.finish_node();
//     named_g.finish_node();
// }

// /// See: https://spec.graphql.org/October2021/#NamedType
// ///
// /// *NamedType*:
// ///     Name
// pub(crate) fn named_type(p: &mut Parser) {
//     let _g = p.start_node(SyntaxKind::NAMED_TYPE);
//     name::name(p);
// }

// #[cfg(test)]
// mod test {
//     use crate::{ast, Parser};

//     #[test]
//     fn it_parses_nested_wrapped_types_in_op_def_and_returns_matching_stringified_doc() {
//         let mutation = r#"
// mutation MyMutation($custId: [Int!]!) {
//   myMutation(custId: $custId)
// }"#;
//         let parser = Parser::new(mutation);
//         let ast = parser.parse();
//         assert!(ast.errors.is_empty());

//         let doc = ast.document();
//         assert_eq!(&mutation, &doc.to_string());

//         for definition in doc.definitions() {
//             if let ast::Definition::OperationDefinition(op_type) = definition {
//                 for var in op_type
//                     .variable_definitions()
//                     .unwrap()
//                     .variable_definitions()
//                 {
//                     if let ast::Type::NamedType(name) = var.ty().unwrap() {
//                         assert_eq!(name.to_string(), "[Int!]!")
//                     }
//                 }
//             }
//         }
//     }

//     #[test]
//     fn stringified_ast_matches_input_with_deeply_nested_wrapped_types() {
//         let mutation = r#"
// mutation MyMutation($a: Int $b: [Int] $c: String! $d: [Int!]!

//     $e: String
//     $f: [String]
//     $g: String!
//     $h: [String!]!
// ) {
//   myMutation(custId: $a)
// }"#;
//         let parser = Parser::new(mutation);
//         let ast = parser.parse();

//         let doc = ast.document();
//         assert_eq!(&mutation, &doc.to_string());
//     }

//     #[test]
//     fn stringified_ast_matches_input_with_deeply_nested_wrapped_types_with_commas() {
//         let mutation = r#"
// mutation MyMutation($a: Int, $b: [Int], $c: String!, $d: [Int!]!,

//     $e: String,
//     $f: [String],
//     $g: String!,
//     $h: [String!]!,
// ) {
//   myMutation(custId: $a)
// }"#;
//         let parser = Parser::new(mutation);
//         let ast = parser.parse();

//         let doc = ast.document();
//         assert_eq!(&mutation, &doc.to_string());
//     }
// }
