#[cfg(test)] mod data;

#[derive(Debug)]
pub enum DirectoryItem {
    Directory {
        name: String,
        size: Option<u64>,
        items: Vec<usize>,
        expanded: bool
    },
    File {
        name: String,
        size: u64,
    }
}

#[derive(PartialEq)]
pub enum CommandState {
    ChangeDir,
    List,
}

pub fn parse_directory_data(data: &[&str]) -> Vec<DirectoryItem> {
    let mut dir_history = vec![0usize];
    let mut items = vec![DirectoryItem::Directory { name: String::from("/"), size: None, items: Vec::new(), expanded: false }];
    let mut state = CommandState::ChangeDir;

    // Parse commands
    for line in data.iter().skip(1) { // Skip nav to /
        let split_line: Vec<&str> = line.split_whitespace().collect();

        match split_line.as_slice() {
            ["$", "cd", "/"] => {
                dir_history.clear();
                dir_history.push(0usize);
                continue;
            },
            ["$", "cd", ".."] => {
                dir_history.pop();
                continue;
            },
            ["$", "cd", dir] => {
                // Find dir index
                if let Some(DirectoryItem::Directory { items: dir_items, .. }) = items.get(*dir_history.last().unwrap()) {
                    for item_idx in dir_items.iter() {
                        let name = match items.get(*item_idx).unwrap() {
                            DirectoryItem::Directory { name, .. } => name,
                            DirectoryItem::File { .. } => continue,
                        };

                        if name.eq(dir) {
                            dir_history.push(*item_idx);
                        }
                    }
                }
            },
            ["$", "ls"] => {
                if let Some(DirectoryItem::Directory { expanded, .. }) = items.get_mut(*dir_history.last().unwrap()) {
                    *expanded = true;
                }

                state = CommandState::List;
                continue;
            },
            ["dir", name] => {
                // Add new dir item
                let item_idx = items.len();
                items.push(DirectoryItem::Directory { name: name.to_string(), size: None, items: Vec::new(), expanded: false });

                if let Some(DirectoryItem::Directory { items, .. }) = items.get_mut(*dir_history.last().unwrap()) {
                    items.push(item_idx);
                }
            },
            [size, name] => {
                // Add new file item
                let item_idx = items.len();
                items.push(DirectoryItem::File { name: name.to_string(), size: size.parse().unwrap() });

                if let Some(DirectoryItem::Directory { items, .. }) = items.get_mut(*dir_history.last().unwrap()) {
                    items.push(item_idx);
                }
            }
            _ => {}
        }

        // Read directory listing?

        if state.eq(&CommandState::List) {

        }

        // Check if current directory has already been ls'd?

    }

    items
}

pub fn print_dir_items(items: &[DirectoryItem]) {
    print_item(items, 0, 0);
}

fn print_item(items: &[DirectoryItem], idx: usize, indent_size: usize) {
    let indent = String::from_utf8(vec![b' '; indent_size * 2]).unwrap();

    match &items[idx] {
        DirectoryItem::File { name, size } => {
            println!("{indent}- {name} (file, size={size})");
        },
        DirectoryItem::Directory { name, items: dir_items, .. } => {
            println!("{indent}- {name} (dir)");

            for item_idx in dir_items.iter() {
                print_item(items, *item_idx, indent_size + 1);
            }
        },
    }
}

pub fn calculate_dir_sizes(items: &mut [DirectoryItem]) {
    calculate_dir_sizes_recurse(items, 0);
}

fn calculate_dir_sizes_recurse(items: &mut [DirectoryItem], idx: usize) -> u64 {
    let sub_items = match &items[idx] {
        DirectoryItem::Directory { size: Some(s), .. } => return *s,
        DirectoryItem::Directory { items, .. } => items.to_owned(),
        DirectoryItem::File { size, .. } => return *size,
    };

    let mut current_size = 0u64;

    // Iterate subdirs
    for sub_item_idx in sub_items {
        current_size += calculate_dir_sizes_recurse(items, sub_item_idx);
    }

    // Update size
    if let DirectoryItem::Directory { size, .. } = &mut items[idx] {
        *size = Some(current_size);
    }

    current_size
}

pub fn find_idices_where<T, F: Fn(&T) -> bool>(items: &[T], case: F) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter(|(i, item)| case(item))
        .map(|(i, _)| i)
        .collect()
}

pub fn get_sum_where_dir_sizes_less_than_100000(items: &mut [DirectoryItem]) -> u64 {
    items
        .iter()
        .fold(0u64, |acc, item| match item {
            DirectoryItem::Directory { size: Some(s), .. } => if *s <= 100_000 {
                acc + s
            } else {
                acc
            },
            _ => acc
        })
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 95437)]
    #[case(TEST_DATA_1, 1423358)]
    fn get_sum_where_dir_sizes_less_than_100000_test<const N: usize>(#[case] raw_data: [&str; N], #[case] expected: u64) {
        let mut items = parse_directory_data(&raw_data);
        calculate_dir_sizes(&mut items);

        let result = get_sum_where_dir_sizes_less_than_100000(&mut items);
        //print_dir_items(&items);

        assert_eq!(expected, result);
    }
}