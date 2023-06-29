fn main() {
    let args: Vec<String> = std::env::args().collect();
    let shrink_or_grow = args.get(1).unwrap();
    let direction = args.get(2).unwrap();
    let percent = args.get(3).unwrap().parse::<f32>().unwrap();

    let mut conn = swayipc::Connection::new().unwrap();

    let workspaces = conn.get_workspaces().unwrap();
    let reference = workspaces.iter().find(|ws| ws.focused).unwrap();

    let ref_size = match reference.layout.as_str() {
        "splitv" => reference.rect.height,
        "splith" => reference.rect.width,
        _ => panic!("Workspace is not a split"),
    };
 
    let px_count = (ref_size as f32 / 100.0 * percent).round();
    let cmd = format!("resize {shrink_or_grow} {direction} {px_count:.0} px");
    println!("Executing: {cmd}");

    conn.run_command(cmd).unwrap();
}
