use std::io::{stdout, Write};
use std::io;
use std::sync::Mutex;
use std::sync::MutexGuard;

static POSITIONS: Mutex<[&str; 10]> = Mutex::new(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
static PLAYER: Mutex<&'static str> = Mutex::new("O");


fn main() { 
   println!("Digite uma posição para jogar"); 

   print_board_init();

   loop {

        let mut pos = POSITIONS.lock().unwrap();

	let check: i32 = check_status(&mut pos);

	if check == 1 {
	   println!("Player {}  won", PLAYER.lock().unwrap());
	   break;
	} else if check == 2 {
	   println!("End game, nobody won");
	   break;
	}

	let position: i32 = get_scan_screen();         

	if position > 0 && position <= 9 {
	   if pos[position as usize] != "X" && pos[position as usize] != "O" {
               let next_player = next_player();
	       pos[position as usize] = next_player;
	    }
	}
	       
        clear_screen();
        print_board(&mut pos);
   }
}


fn print_board_init() {
    println!("{} | {} | {}", 1, 2, 3);
    println!("---   ---");
    println!("{} | {} | {}", 4, 5, 6);
    println!("---   ---");
    println!("{} | {} | {}", 7, 8, 9);
}

fn print_board(pos: &mut MutexGuard<[&str; 10]>) {
    println!("{} | {} | {}", pos[1], pos[2], pos[3]);
    println!("---   ---");
    println!("{} | {} | {}", pos[4], pos[5], pos[6]);
    println!("---   ---");
    println!("{} | {} | {}", pos[7], pos[8], pos[9]);
}


fn next_player() -> &'static str {
    let mut p = PLAYER.lock().unwrap();
    if *p == "O" {
        *p = "X";
    } else {
        *p = "O";
    }
    *p
}

fn check_status(pos: &mut MutexGuard<[&str; 10]>) -> i32 {
   if pos[1] == pos[2] && pos[2] == pos[3] {
        return 1;
   } else if pos[4] == pos[5] && pos[5] == pos[6] {
	return 1;
   } else if pos[7] == pos[8] && pos[8] == pos[9] {
	return 1;
   }

   if pos[1] == pos[4] && pos[4] == pos[7] {
	return 1;
   } else if pos[2] == pos[5] && pos[5] == pos[8] {
	return 1;
   } else if pos[3] == pos[6] && pos[6] == pos[9] {
	return 1;
   }

   if pos[1] == pos[5] && pos[5] == pos[9] {
	return 1;
   } else if pos[3] == pos[5] && pos[5] == pos[7] {
	return 1;
   }

   if pos[1] != "1" && pos[2] != "2" && pos[3] != "3" && pos[4] != "4" && pos[5] != "5" && pos[6] != "6" && pos[7] != "7" && pos[8] != "8" && pos[9] != "9" {
	return 2;
   }

   return -1;
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

fn get_scan_screen() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Invalid input");
    
    return number;
}
