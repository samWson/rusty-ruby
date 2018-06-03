///// ast.rs describes an Abstract Syntax Tree (AST). An AST is an abstract representation of the
///// syntatic structure of a program
//
///// Node is a point in an AST. A node will be an Expression, a Statement, or the Program.
//trait Node {
//    fn token_literal(&self) -> String {
//        if let Some(&String) = self.token {
//            &String
//        }
//    }
//}
//
///// Statement is a Node in an AST. A statement differs from an expression in that it does not
///// produce a value.
//trait Statement: Node {
//    fn statement_node(&self);
//}
//
///// Expression is a Node in an AST. An expression differs from a statement in that it does produce
///// a value.
//trait Expression: Node {
//    fn expression_node(&self);
//}
//
///// AST is an abstract syntax tree. The variants of AST are the nodes at each point in the tree.
//enum AST {
//    /// Program is the root node of every AST.
//    Program {
//        statments: vec<Statement>,
//    },
//    AssignmentStatement {
//        token: Token,
//        name: &Identifier,
//        value: Expression,
//    },
//    Identifier {
//        token: Token,
//        value: String,
//    },
//}
//
//impl Node for AST::Program {
//    fn token_literal(&self) -> String {
//        if statments.len() > 0 {
//            self.statments[0].token_literal()
//        } else {
//            ""
//        }
//    }
//}
//
//impl Statement for AST::AssignmentStatement {
//    fn statement_node(&self) {}
//}
//
//impl Expression for AST::Identifier {
//    fn expression_node(&self) {}
//}
