use anyhow::Result;

fn main() -> Result<()> {
    treeedbgen_souffle::cli::main(tree_sitter_cpp::NODE_TYPES)
}
