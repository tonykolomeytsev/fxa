use std::collections::BTreeSet;

/// Damerau–Levenshtein distance between two words.
/// Used to correct typos when typing an icon/picture name.
///
/// # Arguments
///
/// * `a` - first word to compare
/// * `b` - second word to compare
///
/// > Word order isn't important :)
///
/// # Returns
///
/// Function returns edit distance between two words (char sequences).
/// The smaller the distance, the more the words are similar to each other.
///
/// For more details follow:
/// https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
///
/// # Examples
///
/// ```rust
/// let a = "abc".to_string();
/// let b = "abcd".to_string();
/// assert_eq!(dldistance(&a, &b), 1usize);
///
///
/// let a = "hello".to_string();
/// let b = "hello".to_string();
/// assert_eq!(dldistance(&a, &b), 0usize);
/// ```
pub fn dldistance(a: &String, b: &String) -> usize {
    // 2-d array of integers, dimensions a.len(), b.len()
    let mut d: Vec<Vec<usize>> = a
        .chars()
        .map(|_| b.chars().map(|_| 0usize).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    for i in 0..a.len() {
        d[i][0] = i;
    }
    for j in 0..b.len() {
        d[0][j] = j;
    }

    for i in 1..a.len() {
        for j in 1..b.len() {
            let cost = if a.chars().nth(i).unwrap() == b.chars().nth(j).unwrap() {
                0
            } else {
                1
            };
            d[i][j] = (d[i - 1][j] + 1) // deletion
                .min(d[i][j - 1] + 1) // insertion
                .min(d[i - 1][j - 1] + cost); // substitution

            let curr_a = a.chars().nth(i).unwrap();
            let curr_b = b.chars().nth(j).unwrap();
            let prev_a = a.chars().nth(i - 1).unwrap();
            let prev_b = b.chars().nth(j - 1).unwrap();
            if i > 1 && j > 1 && curr_a == prev_b && prev_a == curr_b {
                d[i][j] = (d[i][j]).min(d[i - 2][j - 2] + 1);
            }
        }
    }

    d[a.len() - 1][b.len() - 1]
}

#[test]
fn test_dldistance_1() {
    let a = "ic_36/fingerprint".to_string();
    let b = "ic_36/fingerprint ".to_string();
    assert_eq!(dldistance(&a, &b), 1usize);
}

#[test]
fn test_dldistance_2() {
    let a = "im_bannr".to_string();
    let b = "img_banner".to_string();
    assert_eq!(dldistance(&a, &b), 2usize);
}

#[test]
fn test_dldistance_3() {
    let a = "ic_16/arrow_back".to_string();
    let b = "ic_16/arrow_back".to_string();
    assert_eq!(dldistance(&a, &b), 0usize);
}

pub fn generate_name_suggections(
    desired_name: &String,
    available_names: &Vec<String>,
) -> Option<Vec<String>> {
    let mut output = BTreeSet::new();

    // First, look for the names that contains desired name
    for name in available_names {
        if name.contains(desired_name) {
            output.insert(name.clone());
        }
    }

    // Second, look for the `similar` names
    for name in available_names {
        if dldistance(&name, &desired_name) < 3usize {
            output.insert(name.clone());
        }
    }

    if output.is_empty() {
        None
    } else {
        // Take only 5 suggestions
        let mut output = output
            .iter()
            .take(5usize)
            .map(|s| s.clone())
            .collect::<Vec<String>>();
        output.sort_by(|a, b| a.cmp(b));
        Some(output)
    }
}
