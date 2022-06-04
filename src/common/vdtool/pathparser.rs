use crate::common::vdtool::error::VectorDrawableError;
use crate::common::vdtool::ir::PathDataNode;

pub fn parse_path(d: &str) -> Result<Vec<PathDataNode>, VectorDrawableError> {
    let value = d.trim();
    let mut output = Vec::new();

    let mut start = 0usize;
    let mut end = 1usize;

    loop {
        if end > value.len() {
            break;
        }

        end = next_start(value, end);
        let s = &value[start..end];
        let current_command = s.chars().nth(0).unwrap();
        let floats = get_floats(s);

        if start == 0 {
            // For the starting command, special handling: add M 0 0 if there is none.
            // This is good for transformation.
            if current_command != 'M' && current_command != 'm' {
                output.push(PathDataNode('M', vec![0f32, 0f32]));
            }
        }

        output.push(PathDataNode(current_command, floats));
        start = end;
        end += 1;
    }

    Ok(output)
}

fn next_start(s: &str, end: usize) -> usize {
    let mut end = end;
    loop {
        if end >= s.len() {
            break;
        }

        let ch = s.chars().nth(end).unwrap();
        // Note that 'e' or 'E' are not valid path commands, but could be used for floating
        // point numbers' scientific notation. Therefore, when searching for next command, we
        // should ignore 'e' and 'E'.
        match ch {
            'e' | 'E' => (),
            'A'..='Z' | 'a'..='z' => return end,
            _ => (),
        }

        end += 1;
    }
    end
}

fn get_floats(s: &str) -> Vec<f32> {
    let command = s.chars().nth(0).unwrap();
    if command == 'z' || command == 'Z' {
        return Vec::new();
    }

    let arc_command = command == 'a' || command == 'A';

    let mut results = (&s[1..s.len()])
        .split(' ')
        .map(|v| v.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    if arc_command {
        // https://www.w3.org/TR/SVG/paths.html#ArcOutOfRangeParameters:
        // If either rx or ry have negative signs, these are dropped;
        // the absolute value is used instead.
        for i in (0..results.len()).step_by(7) {
            results[i] = results[i].abs();
            results[i + 1] = results[i + 1].abs();
        }
    }
    results
}
