
//quarto
use std::io;

fn test(){
	let test1 = [[0,6,0,0],[0,3,0,0],[0,21,0,0],[0,210,0,0]]; //przykładowa tablica do testów
	println!("{:?}",place_on_board(test1, 12));
}

fn place_on_board(mut tab: [[i32;4];4], element: i32) -> [[i32; 4]; 4]
{ 
	loop { //pętla w nieskończoność do podania niezajętego miejsca  
        let mut line = String::new(); //wczytywanie user inputu             
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: usize = parts[0].parse().unwrap();
        let b: usize = parts[1].parse().unwrap();
		if tab[a][b]!=0{               
            println!("Wybrane miejsce jest zajęte!");
        }
        else { //znalezienie poprawnego miejsca
            tab[a][b]=element;
            return tab;
        }
    } 
}

#![allow(unused)]
#![allow(non_snake_case)]

use rand::Rng; // 0.8.5
use std::io;


fn bot(mut tab: [[i32; 4]; 4], element: i32) -> [[i32; 4]; 4] {
    loop {
        let a = rand::thread_rng().gen_range(0..=3);
        let b = rand::thread_rng().gen_range(0..=3);
        if tab[a][b] != 0 {
            continue;
        } else {
            tab[a][b] = element; //tu wstaw element
            return tab;
        }
    }
}

fn print_end(player: i8){
	println!("Wygral gracz {}", player + 1);
}

fn check_if_end_test(){
	//wczytanie tablicy do tesu
	let test1 = [[0,6,0,0],[0,3,0,0],[0,21,0,0],[0,210,0,0]];//4 w jednej kolumnie
	println!("test 1: {}", check_if_end(&test1, 0));// gdy gracz = 0
	
	let test2 = [[5,10,1,7],[0,30,0,0],[0,105,0,0],[0,0,0,14]];//4 w jednym rzedzie
	println!("test2: {}", check_if_end(&test2, 0));// gdy gracz = 0
	
	let test3 = [[15,5,35,6],[105,42,2,14],[1,0,0,0],[30,0,0,0]];//brak
	println!("test 3: {}", check_if_end(&test3, 0));// gdy gracz = 0
	
	let test4 = [[0,1,0,70],[0,0,105,0],[0,30,2,15],[5,0,0,6]];//4 na skosie "/"
	println!("test 4: {}", check_if_end(&test4, 0));// gdy gracz = 0
	
	let test5 = [[10,105,0,0],[21,30,2,5],[0,0,6,0],[0,0,0,70]];//4 na skosie "\"
	println!("test 5: {}", check_if_end(&test5, 0));// gdy gracz = 0
}

fn check_if_end(tab: &[[i32;4];4], player: i8) -> bool{//true - gra toczy 
	
	//sprawdzam czy sa wolne miejsca
	let free = 0;//co jest jesli miejsce jest wolne
	let mut sum = 0;
	for i in 0..4{
		for j in 0..4{
//			print!("{} ", tab[i][j]);
			if tab[i][j] == free{//
				sum += 1;//licze sume wolnych miejsc
			}
		}
//		println!();
	}
	
	if sum == 0{//brak wolnych miejsc
		print_end(player);
		return true;
	}
	
	//skoro sa jeszcze wolnie miejsca sprawdzam czy nie ma 4 takich samych w rzedzie / kolumnie / na skosach
	
	let feature = [2,3,5,7];//mod wlasciwosci
	let mut count = 0;
	let mut free_elements = 0;
	
	//sprawdzam czy sa 4 w rzedzie
	
	for f in feature {//dla kazdej wlasciwosci
//		println!("f = {f}");
		for i in 0..4 { //dla kazdego rzedu
			//sprawdzenie wlasnosci dla wszystkich elementow
			for j in 0..4 {
				if tab[i][j] % f == 0{//jesli jest spelnione 
					count += 1;
				}
				if tab[i][j] == 0{//jesli jest puste
					free_elements += 1;
				}
			}
			
			//sprawdzenie czy sa 4 w tym rzedzie
//			println!("rzad == {i}, count = {count}, free_elements == {free_elements}");
			if free_elements == 0{//jesli wszystkie uzupelnione
				if count == 0 || count == 4{//jesli wszystkie takie same
					//koniec gry
//					println!("rzad {}", i + 1);
					print_end(player);
					return true;
				}
			}
			count = 0;
			free_elements = 0;
		}
	}
	
	//4 w kolumnie
	
	for f in feature {//dla kazdej wlasciwosci
//		println!("f = {f}");
		for j in 0..4 { //dla kazdej kolumny
			//sprawdzenie wlasnosci dla wszystkich elementow
			for i in 0..4 {
				if tab[i][j] % f == 0{//jesli jest spelnione 
					count += 1;
				}
				if tab[i][j] == 0{//jesli jest puste
					free_elements += 1;
				}
			}
			
			//sprawdzenie czy sa 4 w tej kolumnie
//			println!("kolumna == {j}, count = {count}, free_elements == {free_elements}");
			if free_elements == 0{//jesli wszystkie uzupelnione
				if count == 0 || count == 4{//jesli wszystkie takie same
					//koniec gry
//					println!("kolumna {}", j + 1);
					print_end(player);
					return true;
				}
			}
		count = 0;
		free_elements = 0;
		}
	}
	
	//sprawdzenie dla skosu "\"
	for f in feature {//dla kazdej wlasciwosci
		//dla kazdej wartosci na skosie
		for i in 0..4{
			if tab[i][i] % f == 0 {//jesli ma wlasnosc f
				count += 1
			}
			if tab[i][i] == 0 {//jesli jest pusty
				free_elements += 1
			}
		}
//		println!("kolumna == {j}, count = {count}, free_elements == {free_elements}");
		if free_elements == 0{//jesli wszystkie uzupelnione
			if count == 0 || count == 4{//jesli wszystkie takie same
			//koniec gry
//				println!("skos lewy");
				print_end(player);
				return true;
			}
		}
		count = 0;
		free_elements = 0;
	}
	
	//sprawdzenie dla skosu "/"
		
	for f in feature {//dla kazdej wlasciwosci
		//dla kazdej wartosci na skosie
		for i in 0..4{
			if tab[i][3 - i] % f == 0 {//jesli ma wlasnosc f
				count += 1
			}
			if tab[i][3 - i] == 0 {//jesli jest pusty
				free_elements += 1
			}
		}
//		println!("kolumna == {j}, count = {count}, free_elements == {free_elements}");
		if free_elements == 0{//jesli wszystkie uzupelnione
			if count == 0 || count == 4{//jesli wszystkie takie same
			//koniec gry
//				println!("skos prawy");
				print_end(player);
				return true;
			}
		}
		count = 0;
		free_elements = 0;
	}	
	
	return false;
}

fn random_item() -> i32 { 
    let mut rng = rand::thread_rng();

	  let feature = [2,3,5,7]; // mod wlasnosci
	  let mut value = 1; // finalna wartosc wlasnosci
	  for i in 0..4 {
	      let toss: bool = rng.gen_bool(0.5); // losowanie true lub false z takim samym prawdopodobienstwem
//      println!("Wlasciwosc: {}", toss);
        if toss == true { // jesli true dodajemy elementowi wlasnosc
            value *= feature[i]
        }
	  }
	  return value
}

fn main() {
//  let tab: [[i32;4];4] = [[0;4];4];//utworzenie tablicy w ktorej bedzie zapisywany stan gry
//  jesli 0 to puste pole, w innym przypadkach odpowiednio 1*(2)*(3)*(5)*(7) zaleznie od cech elementu 
    println!("Wartosc: {}", random_item());
}
