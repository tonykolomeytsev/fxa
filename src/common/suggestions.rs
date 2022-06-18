use eddie::JaroWinkler;
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub fn generate_name_suggestions(
    desired_name: &String,
    available_names: &Vec<String>,
) -> Option<Vec<String>> {
    let mut output = BTreeSet::new();
    let jarwin = JaroWinkler::new();

    // First, look for the names that contains desired name
    for name in available_names {
        if name.contains(desired_name) {
            output.insert(name.clone());
        }
    }

    // Second, look for the `similar` names
    let mut similar_names = available_names
        .iter()
        .map(|n| (n, jarwin.similarity(n, &desired_name)))
        .collect::<Vec<(&String, f64)>>();
    similar_names.sort_by(|(_, s1), (_, s2)| s2.partial_cmp(s1).unwrap_or(Ordering::Equal));
    similar_names.iter().take(5).for_each(|&(name, _)| {
        output.insert(name.clone());
    });

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
