use std::collections::VecDeque;
use swayipc::{Node, NodeLayout, Connection};

/// Breadth first search for the first node for which `predicate` holds
fn bfsearch<'a>(root: &'a Node, predicate: impl Fn(&'a Node) -> bool)
-> Option<&'a Node>
{
    let mut q = VecDeque::from(vec![root]);

    while let Some(n) = q.pop_front() {
        if predicate(n) {
            return Some(n)
        }

        q.extend(n.nodes.iter());
    };

    None // Never found
}

/// Find a node with `id` in some (sub-)tree
fn find_by_id(root: &Node, id: i64) -> Option<&Node> {
    bfsearch(root, |n| n.id == id)
}

/// Find the highest level node that is focused.
/// This should be the "largest" container that is focused
fn top_focus(root: &Node) -> Option<&Node> {
    bfsearch(root, |n| n.focused)
}

/// Finds the parent of the current focus
fn get_focus_parent<'a>(conn: &mut Connection, tree: &'a Node) -> &'a Node {
    let workspaces = conn.get_workspaces().unwrap();
    // TODO: use tree perhaps?
    let focused_workspace = workspaces.iter()
        .find(|w| w.focused)
        .unwrap();

    let focused_workspace_node = find_by_id(tree, focused_workspace.id).unwrap();
    let focus = top_focus(focused_workspace_node).unwrap();

    let has_as_child = |n: &Node| n.nodes.iter()
        .any(|n|n.id == focus.id);

    bfsearch(tree, has_as_child).unwrap()
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let shrink_or_grow = args.get(1).unwrap();
    let direction = args.get(2).unwrap();
    let percent = args.get(3).unwrap().parse::<f32>().unwrap();

    let mut conn = swayipc::Connection::new().unwrap();
    let tree = conn.get_tree().unwrap();
    let parent = get_focus_parent(&mut conn, &tree);

    let ref_size = match parent.layout {
        NodeLayout::SplitV => parent.rect.height,
        NodeLayout::SplitH => parent.rect.width,
        _ => panic!("Workspace is not a split"),
    };
 
    let px_count = (ref_size as f32 / 100.0 * percent).round();
    let cmd = format!("resize {shrink_or_grow} {direction} {px_count:.0} px");

    conn.run_command(cmd).unwrap();
}
