use crate::types::GitObject;

pub fn ls_tree(tree_sha: &str, name_only: bool) -> anyhow::Result<()> {
    let go = GitObject::from_hash(tree_sha)?;
    go.pretty_print(name_only)?;
    Ok(())
}
