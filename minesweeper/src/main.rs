fn main() {
    let mine: u8= 42;
    let no_mine: u8 = 32;

    let input_minefield = [
        "  *  ".as_bytes(),
        "  *  ".as_bytes(),
        "*****".as_bytes(),
        "  *  ".as_bytes(),
        "  *  ".as_bytes(),
    ];

    let input_lenght = input_minefield.len();
    //print!("{}", input_lenght);

    //Ein 2D Array erstellen zum speichern der minen im Umfeld. Array ist mit 0 gefühlt!
    let mut mines_nearby: Vec<Vec<u8>> = vec![vec![b'0'; input_lenght]; input_lenght]; 
   
    
    for i in 0..input_lenght{
        //println!("{:?}", input_minefield[i]);
        for j in 0..input_lenght{
            //print!("{:?}", input_minefield[i][j]);

        //Hochzählen der Minen in Waagerechter Richtung:
            //Für Feld welches Sich nicht am Rand befindet
            if input_minefield[i][j] == no_mine && j != 0 && j != input_lenght-1{
                if input_minefield[i][j-1] == mine{
                    mines_nearby[i][j] += 1;
                }
                if input_minefield[i][j+1] == mine{
                    mines_nearby[i][j] += 1;
                }

            //Für Feld was sich am linken Rand befindet:
            }else if  input_minefield[i][j] == no_mine && j == 0{
                if input_minefield[i][j+1] == mine{
                    mines_nearby[i][j] += 1;
                }
            
            //für Fekd wwas sich am Rechten Rand befindet:
            }else if  input_minefield[i][j] == no_mine && j == input_lenght-1{
                if input_minefield[i][j-1] == mine{
                    mines_nearby[i][j] += 1;
                }
            }

        //Hochzählen der Vertikalen und Diagonalen Minen:
            if i > 0{

                //Vertikale Abfrage
                if input_minefield[i-1][j] == no_mine && input_minefield[i][j] == mine && j > 0 && j < input_lenght-1 {
                    mines_nearby[i-1][j] +=1;


                //Für Feld was sich am linken Rand befindet:
                }else if  input_minefield[i-1][j] == no_mine && input_minefield[i][j] == mine && j == 0{
                    if input_minefield[i][j+1] == mine{
                        mines_nearby[i-1][j] += 1;
                    }
                
                //für Fekd wwas sich am Rechten Rand befindet:
                }else if  mines_nearby[i-1][j] == mine && input_minefield[i][j] == mine && j == input_lenght-1{
                    if input_minefield[i][j-1] == mine{
                        mines_nearby[i-1][j] += 1;
                    }
                }

                

            }

            if i < input_lenght-1{

                //Diagonale Abfrage
                if input_minefield[i+1][j] == no_mine && input_minefield[i][j] == mine && j > 0 && j < input_lenght-1 {
                    mines_nearby[i+1][j] +=1;


                //Für Feld was sich am linken Rand befindet:
                }else if  input_minefield[i+1][j] == no_mine && input_minefield[i][j] == mine && j == 0{
                    if input_minefield[i][j+1] == mine{
                        mines_nearby[i+1][j] += 1;
                    }
                
                //für Fekd wwas sich am Rechten Rand befindet:
                }else if  input_minefield[i+1][j] == no_mine && input_minefield[i][j] == mine && j == input_lenght-1{
                    if input_minefield[i][j-1] == mine{
                        mines_nearby[i+1][j] += 1;
                    }
                }

                

            }


        }
        //println!("");
    }

    for row in &mines_nearby {
        println!("{}", String::from_utf8_lossy(row));
    }
    

}

