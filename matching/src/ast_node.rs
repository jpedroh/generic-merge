use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Debug)]
pub struct ASTNode {
    pub kind: String,
    pub identifier: String,
    pub body: String,
    pub children: Vec<ASTNode>,
}

impl Hash for ASTNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.identifier.hash(state);
        self.body.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::ASTNode;

    #[test]
    fn i_can_create_an_ast_node() {
        let result = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![],
        };
        assert_eq!(result.kind, "class_declaration");
        assert_eq!(result.identifier, "Main");
        assert_eq!(
            result.body,
            "public class Main { public static void main(){} }"
        );
        assert_eq!(result.children.len(), 0);
    }

    #[test]
    fn i_can_create_an_ast_node_with_children() {
        let result = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("main"),
                body: String::from("public static void main(){}"),
                children: vec![],
            }],
        };
        assert_eq!(result.kind, "class_declaration");
        assert_eq!(result.identifier, "Main");
        assert_eq!(
            result.body,
            "public class Main { public static void main(){} }"
        );
        assert_eq!(result.children.len(), 1);
    }
}
