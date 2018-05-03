use std::io::Write;
use std::path::Path;
use std::fs::{self};
use std::io;
use std::collections::HashSet;


static CHILD_PREFIX: &'static str = "├──";
static LAST_CHILD_PREFIX : &'static str = "└──";


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 && args.len() > 3{
        writeln!(std::io::stderr(),
                 "Usage: tree PATH")
            .unwrap();
        writeln!(std::io::stderr(),
                 "Example: {} path/to/folder [:depth]",
                 args[0])
            .unwrap();
        std::process::exit(1);
    }

    let root_path = Path::new(&args[1]);
    let root_hash_set: HashSet<u32> = HashSet::new();
    let mut max_depth = 2;

    if args.len() == 3 {
        max_depth = args[2].parse::<u32>().unwrap();
    }
    print_subtree(&root_path, 0, root_hash_set, 0, max_depth).expect("Could not print subtree");
}


fn print_subtree(path: &Path, indent: u32, last_child_indexes: HashSet<u32>, current_depth: u32, max_depth: u32) -> io::Result<()> {
    if current_depth > max_depth {
        return Ok(());
    }

    if current_depth == 0 {
        let path_str = path.to_str().unwrap();
        println!("{}", path_str);
    }

    if path.is_dir() {
        let num_children = fs::read_dir(path)?.count();
        for (i, entry) in fs::read_dir(path)?.enumerate() {
            let is_last_child = i == num_children - 1;

            let entry = entry?;
            let filename = entry.file_name().into_string().unwrap();
            let prefix = create_prefix(indent, is_last_child, &last_child_indexes); 

            println!("{} {}", prefix, filename);

            let child_path = entry.path();
            let child_path_as_path = child_path.as_path();

            if is_last_child {
                let updated_indexes = update_indexes(&last_child_indexes, current_depth);
                print_subtree(child_path_as_path, indent+1, updated_indexes, current_depth+1, max_depth)?;
            } else {
                print_subtree(child_path_as_path, indent+1, last_child_indexes.clone(), current_depth+1, max_depth)?;

            }
        }
    }

    Ok(())
}

fn create_prefix(indent: u32, is_last_child: bool, last_child_indexes: &HashSet<u32>) -> String {
    let mut prefix = String::new();
    for indent_level in 0..indent {
        if last_child_indexes.contains(&indent_level) {
            prefix.push_str("    ");
        } else {
            prefix.push_str("|   ");
        }

    }

    if is_last_child {
        prefix.push_str(LAST_CHILD_PREFIX);
    } else {
        prefix.push_str(CHILD_PREFIX);
    }

    prefix
}

fn update_indexes(last_child_indexes: &HashSet<u32>, depth: u32) -> HashSet<u32> {
    let mut new_last_child_indexes = last_child_indexes.clone();
    new_last_child_indexes.insert(depth);
    new_last_child_indexes
}