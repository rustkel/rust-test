#[allow(dead_code)]

/// Splitter helper function to split strings by several substrings.
/// Substrings in *include* will be kept as a new split,
/// whereas substrings in *exclude* will be removed

pub fn split(string: String, include: Vec<&str>) -> Vec<String> {
    let mut pos: Vec<i32> = vec![0, string.len() as i32];

    match_occurrences(include, &string, &mut pos);
    pos.sort_unstable();

    let mut result = vec![];
    for (i, j) in pos.iter().zip(pos.iter().skip(1)) {
        if *j == *i {
            continue;
        }
        let x = (*i).abs() as usize;
        let y = (*j).abs() as usize;
        result.push(string[x..y].to_string());
    }
    result
}

/// Find all matches of the strings in vector.
/// Add the start and end to the given vector of positions.
/// If we want to exclude the pattern, we label the end as negative
/// to be processed later.
fn match_occurrences(vector: Vec<&str>, string: &String, pos: &mut Vec<i32>) {
    for i in vector {
        string.match_indices(i).for_each(|x| {
            pos.push(x.0 as i32);
            pos.push((x.0 + x.1.len()) as i32);
        });
    }
}
