// The standard environment for expr-lang.

use super::id_table::IdentificationTable;
use crate::front::ast::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref STDENV: HashMap<&'static str, Decl> = {
        let mut m = HashMap::new();
        m.insert("false", Decl::ConstDecl(ConstDecl::BoolLiteral(false)));
        m.insert("true", Decl::ConstDecl(ConstDecl::BoolLiteral(true)));
        m.insert(
            "unary_plus",
            Decl::OperatorDecl(OperatorDecl::UnaryOperatorDecl(UnaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "unary_minus",
            Decl::OperatorDecl(OperatorDecl::UnaryOperatorDecl(UnaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "bitwise_not",
            Decl::OperatorDecl(OperatorDecl::UnaryOperatorDecl(UnaryOperatorDecl::new(
                Type::BoolType,
                Type::BoolType,
            ))),
        );

        m.insert(
            "logical_not",
            Decl::OperatorDecl(OperatorDecl::UnaryOperatorDecl(UnaryOperatorDecl::new(
                Type::BoolType,
                Type::BoolType,
            ))),
        );

        m.insert(
            "add",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );

        m.insert(
            "add_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );

        m.insert(
            "assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::AnyType,
                Type::AnyType,
                Type::AnyType,
            ))),
        );

        m.insert(
            "bitwise_and",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );

        m.insert(
            "bitwise_and_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );

        m.insert(
            "bitwise_or",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );

        m.insert(
            "bitwise_or_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );

        m.insert(
            "bitwise_xor",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "bitwise_xor_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "div",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "div_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "equal",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::AnyType,
                Type::AnyType,
                Type::AnyType,
            ))),
        );
        m.insert(
            "greater_than",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "greater_than_or_equal",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "left_shift",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "left_shift_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "less_than",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "less_than_or_equal",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "logical_and",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::BoolType,
                Type::BoolType,
                Type::BoolType,
            ))),
        );
        m.insert(
            "logical_and_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::BoolType,
                Type::BoolType,
                Type::BoolType,
            ))),
        );
        m.insert(
            "logical_or",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::BoolType,
                Type::BoolType,
                Type::BoolType,
            ))),
        );
        m.insert(
            "logical_or_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::BoolType,
                Type::BoolType,
                Type::BoolType,
            ))),
        );
        m.insert(
            "mod",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "mod_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "mul",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "mul_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "not_equal",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::AnyType,
                Type::AnyType,
                Type::AnyType,
            ))),
        );
        m.insert(
            "right_shift",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "right_shift_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "sub",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );
        m.insert(
            "sub_assign",
            Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(BinaryOperatorDecl::new(
                Type::IntType,
                Type::IntType,
                Type::IntType,
            ))),
        );

        m
    };
}

/// Loads the standard environment at level 0.
pub fn load_stdenv(id_table: &mut IdentificationTable) {
    id_table.save_attr("bitwise_not", STDENV.get("bitwise_not").unwrap());
    id_table.save_attr("logical_not", STDENV.get("logical_not").unwrap());

    id_table.save_attr("unary_plus", STDENV.get("unary_plus").unwrap());

    id_table.save_attr("unary_minus", STDENV.get("unary_minus").unwrap());

    id_table.save_attr("add", STDENV.get("add").unwrap());
    id_table.save_attr("add_assign", STDENV.get("add_assign").unwrap());
    id_table.save_attr("bitwise_and", STDENV.get("bitwise_and").unwrap());

    id_table.save_attr(
        "bitwise_and_assign",
        STDENV.get("bitwise_and_assign").unwrap(),
    );

    id_table.save_attr("bitwise_or", STDENV.get("bitwise_or").unwrap());
    id_table.save_attr(
        "bitwise_or_assign",
        STDENV.get("bitwise_or_assign").unwrap(),
    );
    id_table.save_attr("bitwise_xor", STDENV.get("bitwise_xor").unwrap());
    id_table.save_attr(
        "bitwise_xor_assign",
        STDENV.get("bitwise_xor_assign").unwrap(),
    );
    id_table.save_attr("div", STDENV.get("div").unwrap());
    id_table.save_attr("div_assign", STDENV.get("div_assign").unwrap());
    id_table.save_attr("equal", STDENV.get("equal").unwrap());
    id_table.save_attr("greater_than", STDENV.get("greater_than").unwrap());
    id_table.save_attr(
        "greater_than_or_equal",
        STDENV.get("greater_than_or_equal").unwrap(),
    );
    id_table.save_attr("left_shift", STDENV.get("left_shift").unwrap());
    id_table.save_attr(
        "left_shift_assign",
        STDENV.get("left_shift_assign").unwrap(),
    );
    id_table.save_attr("less_than", STDENV.get("less_than").unwrap());
    id_table.save_attr(
        "less_than_or_equal",
        STDENV.get("less_than_or_equal").unwrap(),
    );
    id_table.save_attr("logical_and", STDENV.get("logical_and").unwrap());
    id_table.save_attr(
        "logical_and_assign",
        STDENV.get("logical_and_assign").unwrap(),
    );
    id_table.save_attr("logical_or", STDENV.get("logical_or").unwrap());
    id_table.save_attr(
        "logical_or_assign",
        STDENV.get("logical_or_assign").unwrap(),
    );
    id_table.save_attr("mod", STDENV.get("mod").unwrap());
    id_table.save_attr("mod_assign", STDENV.get("mod_assign").unwrap());
    id_table.save_attr("mul", STDENV.get("mul").unwrap());
    id_table.save_attr("mul_assign", STDENV.get("mul_assign").unwrap());
    id_table.save_attr("not_equal", STDENV.get("not_equal").unwrap());
    id_table.save_attr("right_shift", STDENV.get("right_shift").unwrap());
    id_table.save_attr(
        "right_shift_assign",
        STDENV.get("right_shift_assign").unwrap(),
    );
    id_table.save_attr("sub", STDENV.get("sub").unwrap());
    id_table.save_attr("sub_assign", STDENV.get("sub_assign").unwrap());
}
