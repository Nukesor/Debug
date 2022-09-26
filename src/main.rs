use comfy_table::presets::*;
use comfy_table::*;

fn main() {
    let header = vec!["a\n---\ni64", "b\n---\ni64", "b_squared\n---\ni64"];
    let rows = vec![
        vec!["1", "2", "4.0"],
        vec!["3", "4", "16.0"],
        vec!["5", "6", "36.0"],
    ];

    let mut table = Table::new();

    let table = table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(100);

    table.set_header(header).add_rows(rows);

    println!("{}", table);
}
