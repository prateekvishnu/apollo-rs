use crate::{
    parser::grammar::{input, name, value},
    Parser, SyntaxKind, TokenKind, S, T,
};

// fn f64_const(
//     u: &mut Unstructured,
//     _: &Module,
//     builder: &mut CodeBuilder,
// ) -> Result<Instruction> {
//     let x = u.arbitrary::<f64>()?;
//     builder.push_operands(&[ValType::F64]);
//     Ok(Instruction::F64Const(x))
// }

/// See: https://spec.graphql.org/October2021/#Argument
///
/// *Argument*:
///    Name **:** Value
pub(crate) fn argument(p: &mut Parser, mut is_argument: bool) {

    // if let Some(TokenKind::Name) = p.peek() {
    //     let guard = p.start_node(SyntaxKind::ARGUMENT);
    //     name::name(p);
    //     if let Some(T![:]) = p.peek() {
    //         p.bump(S![:]);
    //         value::value(p);
    //         is_argument = true;
    //         if p.peek().is_some() {
    //             guard.finish_node();
    //             return argument(p, is_argument);
    //         }
    //     }
    // }
    // if !is_argument {
    //     p.err("expected an Argument");
    // }
}

/// See: https://spec.graphql.org/October2021/#Arguments
///
/// *Arguments*:
///    **(** Argument* **)**
pub(crate) fn arguments(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::ARGUMENTS);
    p.bump(S!['(']);
    argument(p, false);
    p.expect(T![')'], S![')']);
}

/// See: https://spec.graphql.org/October2021/#ArgumentsDefinition
///
/// *ArgumentsDefinition*:
///     **(** InputValueDefinition* **)**
pub(crate) fn arguments_definition(p: &mut Parser) -> Node {
    // TODO should be a type
    let mut arguments_def = Vec::new();
    let arb_num = u.arbitrary::<usize>().unwrap();
    for _ in 0..=arb_num {
        let input_value = InputValue::new(
            limited_string(20, u),
            ty::ty(u, builder).try_into_type().unwrap(),
        );
        arguments_def.push(input_value);
    }
    // let _g = p.start_node(SyntaxKind::ARGUMENTS);
    // p.bump(S!['(']);
    // input::input_value_definition(p, false);
    // p.expect(T![')'], S![')']);
}
