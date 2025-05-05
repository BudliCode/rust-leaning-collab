//use std::{io::empty, vec};

use std::collections::HashSet;
//use std::ffi::OsStr;
//Zum lesen von Datein:
//use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn main() -> io::Result<()> {

    //Durchlaufen der Test Files
    let _test_result = run_tests();
    Ok(())
    
}

/*
    Funktionen für das Minesweeper
*/

fn annotate(minefield: &[&str]) -> Vec<String> {

    // Konvertiere das Minefield in ein Vec<Vec<u8>>.
    let input_minefield: Vec<Vec<u8>> = minefield
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();

    //Handeling, bei leeren Input
    if input_minefield.is_empty(){
        let vec:Vec<String> = Vec::new();
        return vec;
    }

    //Umrandung des Minenfeld hinzufügen
    let minefield_with_spacer = add_spacer_to_minefield(input_minefield);

    //Minen Berechnen
    let output_minefield_with_spacer = calculate_mines_count(minefield_with_spacer);
    
    //Umrandung wieder entfernen:
    let output_minefield_without_spacer = remove_spacer_from_minefield(output_minefield_with_spacer);

    output_minefield_without_spacer
}

fn calculate_mines_count(minefield: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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
                if bomb_count != 0{
                    field_with_mine_count[i][j] = b'0' + bomb_count;
                }else {
                    field_with_mine_count[i][j] = b' ';
                }
                
            }
        }
    }

    field_with_mine_count
}

fn add_spacer_to_minefield(minefield: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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

/* 
    Funktionen zum Testen aus Datein
*/ 

fn read_minefiel_from_file(file_path: &Path) -> io::Result<Vec<String>>{

    let mut output_vec: Vec<String> = Vec::new();

    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);

    for line in file_reader.lines() {
        output_vec.push(line?);

    }

    Ok(output_vec)

}  

fn test_files(path: &Path) -> io::Result<Vec<String>> {
    let mut file_names = HashSet::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                let stem_str = stem.to_string_lossy();

                if stem_str != ".DS_Store" {
                    file_names.insert(stem_str.to_string());
                }
            }
        }
    }

    Ok(file_names.into_iter().collect())
}

fn run_tests() -> io::Result<()> {
    let test_dir_path = Path::new("system_test\\test_files");

    // ? entpackt hier ein io::Result<Vec<String>>, wie du richtig erkannt hast
    let test_files = test_files(test_dir_path)?;

    for test in test_files {
        // Original-Dateipfad mit ".mines"-Endung
        let mines_path = test_dir_path.join(format!("{}.mines", test));
        //println!("Vorgabe: {:?}", mines_path);

        let file_minefield = read_minefiel_from_file(&mines_path)?;

        // Erwarteter Dateipfad mit ".expected"-Endung
        let expected_path = test_dir_path.join(format!("{}.expected", test));
        // println!("Erwartet: {:?}", expected_path);

        let expected = read_minefiel_from_file(&expected_path)?;

        assert_eq!(minefield_calculation(file_minefield), expected);
    }

    Ok(())
}

fn minefield_calculation(minefield: Vec<String>) -> Vec<String> {

    // Konvertiere das Minefield in ein Vec<Vec<u8>>.
    let input_minefield: Vec<Vec<u8>> = minefield
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();

    //Handeling, bei leeren Input
    if input_minefield.is_empty(){
        let vec:Vec<String> = Vec::new();
        return vec;
    }

    //Umrandung des Minenfeld hinzufügen
    let minefield_with_spacer = add_spacer_to_minefield(input_minefield);

    //Minen Berechnen
    let output_minefield_with_spacer = calculate_mines_count(minefield_with_spacer);
    
    //Umrandung wieder entfernen und fertiges Minefeld zurückgeben:
    remove_spacer_from_minefield(output_minefield_with_spacer)
}

/*
    Test Funktion für das reine Minesweeper
*/

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
            " 2*2 ".to_string(),
            " 111 ".to_string(),
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
        "     ".to_string(),
        "     ".to_string(),
        "     ".to_string(),
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

    #[test]
    fn wrong_input(){
        let input = &[];
        let expected: &[&str] = &[];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }


}
