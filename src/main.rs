use std::io::stdin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameControl {
    Cross, 
    Circle, 
    Unknown, 
    WaitRestart,
}

fn main() -> ! {
    let mut game_control = GameControl::Unknown;    
    let mut top_level = 0; 
    const WIDTH: usize = 10; 
    const HEIGHT: usize = 10; 
    let mut crosses = vec![Vec::with_capacity(HEIGHT); WIDTH]; 
    let mut input = String::new(); 
    let mut prompt = String::new(); 
    let mut cross_wins = true; 
    let stdin = stdin(); 
    loop {
        show_view(top_level + 3, &crosses); 
        show_hint(game_control, &prompt); 
        prompt.clear(); 
        input.clear();
        stdin.read_line(&mut input).unwrap(); 
        let input = input.trim(); 
        if input == "R" {
            game_control = GameControl::Unknown; 
            top_level = 0; 
            crosses.iter_mut().for_each(|f| f.clear()); 
        } else if input == "r" {
            if let GameControl::WaitRestart = game_control {
                game_control = GameControl::Unknown; 
                top_level = 0; 
                crosses.iter_mut().for_each(|f| f.clear()); 
            } else {
                prompt = String::from("It's invalid to restart when the game is not over. "); 
            }
        } else if input == "X" || input == "x" {
            if game_control == GameControl::Cross {
                prompt = String::from("Yes, it's X's turn. ");
            } else if game_control == GameControl::Circle {
                prompt = String::from("No, it's O's turn. ");
            }
        } else if input == "O" || input == "o" {
            if game_control == GameControl::Circle {
                prompt = String::from("Yes, it's O's turn. ");
            } else if game_control == GameControl::Cross {
                prompt = String::from("No, it's X's turn. ");
            } 
        } else {
            if input.len() != 2 {
                prompt = String::from("Invalid input, use 'X1' or 'O1' to place chess at col 1."); 
                continue ; 
            }
            let mut cs = input.chars();
            let c = cs.next().unwrap(); 
            let is_cross = match c {
                'X' | 'x' => true, 
                'O' | 'o' => false, 
                _ => {
                    prompt = String::from("Invalid input, use 'X' or 'O' to express your chess. ");
                    continue ; 
                }
            };
            let n = cs.next().unwrap(); 
            let n = n.to_digit(10); 
            let Some(n) = n else {
                prompt = String::from("Invalid input, use '0' ~ '9' to set column. ");
                continue ; 
            };
            let n = n as usize; 
            if crosses[n].len() >= HEIGHT {
                prompt = String::from("Invalid input, the column is full. ");
                continue ; 
            } 
            if is_cross && game_control == GameControl::Circle {
                prompt = String::from("Invalid input, it's O's turn. ");
                continue ; 
            } else if !is_cross && game_control == GameControl::Cross {
                prompt = String::from("Invalid input, it's X's turn. ");
                continue ; 
            } 
            crosses[n].push(is_cross); 
            top_level = top_level.max(crosses[n].len());
            let mut win = false; 
            let current = is_cross; 
            let pos_origin_x = n; 
            let pos_origin_y = crosses[n].len() - 1; 
            let mut pos_x;
            let mut pos_y;
            const offset: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)]; 
            for (ox, oy) in offset {
                let mut cnt = 1; 
                pos_x = pos_origin_x; 
                pos_y = pos_origin_y; 
                loop {
                    let pos = pos_offset(pos_x, pos_y, ox, oy); 
                    let Some((px, py)) = pos else {
                        break ; 
                    }; 
                    let Some(c) = pos_fetch(px, py, &crosses) else {
                        break; 
                    };
                    if c == current {
                        cnt += 1; 
                        pos_x = px; 
                        pos_y = py; 
                    } else {
                        break ; 
                    } 
                }
                pos_x = pos_origin_x; 
                pos_y = pos_origin_y; 
                loop {
                    let pos = pos_offset(pos_x, pos_y, -ox, -oy); 
                    let Some((px, py)) = pos else {
                        break ; 
                    }; 
                    let Some(c) = pos_fetch(px, py, &crosses) else {
                        break; 
                    };
                    if c == current {
                        cnt += 1; 
                        pos_x = px; 
                        pos_y = py; 
                    } else {
                        break ; 
                    } 
                } 
                if cnt >= 4 {
                    win = true; 
                    break ;  
                }
            }
            if win {
                game_control = GameControl::WaitRestart; 
                if current {
                    prompt = String::from("X wins! "); 
                } else {
                    prompt = String::from("O wins! "); 
                } 
                cross_wins = current; 
            } else {
                game_control = if current { 
                    GameControl::Circle
                } else {
                    GameControl::Cross
                }; 
            }
        }
    } 
}

pub fn show_view(level_suggestion: usize, crosses: &[Vec<bool>]) {
    const HALF_WINDOW_HEIGHT: usize = 50;
    for _ in level_suggestion..HALF_WINDOW_HEIGHT {
        println!(); 
    }
    println!("|---------------------------------------|"); 
    for i in (0..level_suggestion).rev() {
        let p = crosses.iter().map(|c| match c.get(i) {
            Some(true) => 'X', 
            Some(false) => 'O', 
            None => ' ', 
        }); 
        print!("|"); 
        for p in p {
            print!(" {} |", p);
        }
        println!();
    }
    println!("|---|---|---|---|---|---|---|---|---|---|"); 
    println!("| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |"); 
    println!("|---|---|---|---|---|---|---|---|---|---|"); 
}

pub fn show_hint(cur: GameControl, prompt: &str) {
    match cur {
        GameControl::Cross => {
            println!("{prompt}Now is X's turn, please input the position you want to place your chess: ");
        }
        GameControl::Circle => {
            println!("{prompt}Now is O's turn, please input the position you want to place your chess: "); 
        }
        GameControl::Unknown => {
            println!("{prompt}Freely choose your position to place your chess: ");
        }
        GameControl::WaitRestart => {
            println!("{prompt}Game over, please input 'r' to restart the game: "); 
        }
    }
}

pub fn pos_fetch(pos_x: usize, pos_y: usize, crosses: &[Vec<bool>]) -> Option<bool> {
    crosses.get(pos_x).and_then(|c| c.get(pos_y).copied())
} 

pub fn pos_offset(pos_x: usize, pos_y: usize, offset_x: isize, offset_y: isize) -> Option<(usize, usize)> {
    let pos_x = pos_x as isize; 
    let pos_y = pos_y as isize; 
    let offset_x = offset_x as isize; 
    let offset_y = offset_y as isize; 
    let pos_x = pos_x + offset_x; 
    let pos_y = pos_y + offset_y; 
    if pos_x < 0 || pos_y < 0 {
        return None; 
    }
    let pos_x = pos_x as usize; 
    let pos_y = pos_y as usize; 
    Some((pos_x, pos_y))
} 