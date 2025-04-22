use std::vec;

fn main() {

    
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Konvertiere das Minefield in ein Vec<Vec<u8>>.
    let input_minefield: Vec<Vec<u8>> = minefield
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();

    // Hier fügst du den Spacer hinzu. Achte darauf, dass du diese Funktion definierst, falls sie notwendig ist.
    let minefield_with_spacer = add_spacer_to_minefield(input_minefield);

    //let x_len = minefield[0].len();
    //let y_len = minefield.len();

    let output_minefield_with_spacer = calculate_mines_count(minefield_with_spacer);
    // Falls du mit dem modifizierten Minefield etwas tun möchtest, hier ein Platzhalter:
    let output_minefield_without_spacer = remove_spacer_from_minefield(output_minefield_with_spacer);

    output_minefield_without_spacer
}

pub fn calculate_mines_count(minefield: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let MINE: u8 = b'*';
    let NO_MINE: u8 = b' ';
    
    // Ausgabe-Vektor mit gleichen Maßen, initialisiert mit dem Originalfeld
    let mut field_with_mine_count = minefield.clone();

    /*
        ///////
        /.*.*./
        /..*../
        /..*../
        /...../
        ///////
     */

    // Nur im Inneren suchen (kein Rand!)
    for (i, lines) in minefield.iter().enumerate() {
        for (j, char) in lines.iter().enumerate() {
            if *char == NO_MINE {
                let mut bomb_count = 0;

                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue; // skip center
                        }

                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        if minefield[ni as usize][nj as usize] == MINE {
                            bomb_count += 1;
                        }
                    }
                }

                // Zahl als ASCII-Zeichen (b'0' + bomb_count)
                field_with_mine_count[i][j] = b'0' + bomb_count;
            }
        }
    }

    field_with_mine_count
}

pub fn add_spacer_to_minefield(minefield: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let spacer_char: u8 = b'/'; // "/" als ASCII
    let padding = 1;
    let minefield_x_len = minefield.len();
    let minefield_y_len = minefield[0].len();
    let new_x_len = minefield_x_len + 2 * padding;
    let new_y_len = minefield_y_len + 2 * padding;

    let mut minefield_with_padding = vec![vec![0u8; new_y_len]; new_x_len];

    for i in 0..new_x_len {
        for j in 0..new_y_len {
            // Rahmen setzen
            if i == 0 || i == new_x_len - 1 || j == 0 || j == new_y_len - 1 {
                minefield_with_padding[i][j] = spacer_char;
            } else {
                // Innenbereich mit Originaldaten befüllen
                minefield_with_padding[i][j] = minefield[i - 1][j - 1];
            }
        }
    }

    minefield_with_padding
}

fn remove_spacer_from_minefield(minefield: Vec<Vec<u8>>) -> Vec<String> {
    let x_len = minefield.len();
    let y_len = minefield[0].len();

    let mut trimmed = vec![];
    for i in 1..x_len - 1 {
        // Schneide die Randspalten ab
        let row_slice = &minefield[i][1..y_len - 1];
        // Konvertiere zu String (erst &[u8] → String)
        let row_string = String::from_utf8_lossy(row_slice).to_string();
        trimmed.push(row_string);
    }
    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_01() {
        let input = [
            " * * ",
            "  *  ",
            "  *  ",
            "     ",
        ];

        let expected = vec![
            "1*3*1".to_string(),
            "13*31".to_string(),
            "02*20".to_string(),
            "01110".to_string(),
        ];

        let result = annotate(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_02() {
        let input = [
            "    *    ",
            " * * * * ",
            "*********",
            "         ",
        ];

        let expected = vec![
            "1122*2211".to_string(),
            "3*5*6*5*3".to_string(),
            "*********".to_string(),
            "233333332".to_string(),
        ];

        let result = annotate(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_field() {
        let input = [
        "     ",
        "     ",
        "     ",
        ];

        let expected = vec![
        "00000".to_string(),
        "00000".to_string(),
        "00000".to_string(),
        ];

        let result = annotate(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_mines() {
        let input = [
        "****",
        "****",
        "****",
        ];

        let expected = vec![
        "****".to_string(),
        "****".to_string(),
        "****".to_string(),
        ];

        let result = annotate(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_alternating_row_mines() {
        let input = [
        "* * * * *",
        ];

        let expected = vec![
        "*2*2*2*2*".to_string(),
        ];

        let result = annotate(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_column_with_mine() {
        let input = [
        " ",
        "*",
        " ",
        ];

        let expected = vec![
        "1".to_string(),
        "*".to_string(),
        "1".to_string(),
        ];

        let result = annotate(&input);
        assert_eq!(result, expected);
    }


}
