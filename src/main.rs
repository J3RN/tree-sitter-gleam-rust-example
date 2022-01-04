use tree_sitter::{Parser, Tree, TreeCursor};

fn main() {
    let source_code = "pub fn main() { \"Hello, World!\" }";

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_gleam::language())
        .expect("Error loading gleam language");
    let tree = parser.parse(source_code, None).unwrap();
    let mut cursor = Tree::walk(&tree);

    walk(&mut cursor);
    println!("{:?}", tree);
}

fn walk(cursor: &mut TreeCursor) {
    println!("{:?}", cursor.node());

    if cursor.goto_first_child() {
        walk(cursor);

        while cursor.goto_next_sibling() {
            walk(cursor);
        }

        cursor.goto_parent();
    }
}
