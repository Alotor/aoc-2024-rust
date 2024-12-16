use std::convert::TryFrom;

// Safely access the array of inputs
pub fn get2d(input: &Vec<Vec<char>>, i: i32, j: i32) -> &char {
    if let Ok(i) = usize::try_from(i) {
        if let Some(v2) = input.get(i) {
            if let Ok(j) = usize::try_from(j) {
                v2.get(j).unwrap_or(&' ')
            } else {
                &' '
            }
        } else {
            &' '
        }
    } else {
        &' '
    }
}

pub fn get_m<'a>(input: &'a Vec<Vec<u32>>, i: i32, j: i32, default: &'a u32) -> &'a u32 {
    if let Ok(i) = usize::try_from(i) {
        if let Some(v2) = input.get(i) {
            if let Ok(j) = usize::try_from(j) {
                v2.get(j).unwrap_or(default)
            } else {
                default
            }
        } else {
            default
        }
    } else {
        default
    }
}

#[test]
fn test_get() {
    let input =
        vec![
            vec!['S', 'M', 'X', 'X', 'S', 'M'],
            vec!['S', 'S', 'A', 'M', 'X', 'M'],
            vec!['S', 'A', 'X', 'M', 'A', 'A'],
            vec!['X', 'M', 'A', 'S', 'M', 'S'],
            vec!['X', 'X', 'S', 'A', 'M', 'S'],
        ];

    assert_eq!(get2d(&input, 0, 0), &'S');
    assert_eq!(get2d(&input, -1, 0), &' ');
    assert_eq!(get2d(&input, 19, 0), &' ');
    assert_eq!(get2d(&input, 0, 1), &'M');
    assert_eq!(get2d(&input, 0, 2), &'X');
    assert_eq!(get2d(&input, 0, -1), &' ');
    assert_eq!(get2d(&input, 0, 20), &' ');
}
