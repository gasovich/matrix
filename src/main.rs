use std::fs::File;
use std::io::{Read, Write};

struct Matrix {
	v: Vec<f64>,
	row: usize,
	col: usize,
}

impl Matrix {
	fn from_file(path: &str) -> Matrix {
		let mut list = Matrix {
			v: Vec::new(),
			row: 0,
			col: 0,
		};
		
		list.row = 0;
		list.col = 0;
		let mut res:f64 = 0.0;// Буфер вещественного числа при преобразовании строки в число
		let mut f = File::open(path).expect("Error openng file"); // Открываем файл для чтения
		let mut file_data = String::new(); // В эту строку будет считано содержимое файла
		
		f.read_to_string(&mut file_data).expect("Error reading file"); // Чтение файла
		
		// Извлекаем из строки числа и записываем их в вектор значений
		for line in file_data.split("\n") { // Разбиваем файл на сроки
			list.row += 1;		
			for word in line.split(";") { // Разбиваем строки на слова
				list.col += 1;
				let k: Result<f64, _> = word.trim().parse(); // Преобразуем слово в число
				match k { // Если преобразование в число прошло успешно
					Ok(_) => {
						res = k.unwrap(); // то записываем результат в буфер
					},
					Err(_) => {}
				}
				list.v.push(res); // Кладем содержимое буфера в вектор значений матрицы
			}
		}
		
		list.v.pop();
		list.row -= 1;
		list.col /= list.row;
		list
	}
	
	fn get(&self, i: usize, j: usize) -> Option<f64> {
		if i < self.row && j < self.col {
			return Some(self.v[j * self.row + i])
		} else {
			return None
		}
	}

	fn set(&mut self, i: usize, j: usize, x: f64) -> Option<f64> {
		if i < self.row && j < self.col {
			self.v[j * self.row + i] = x;
			Some(x)
		} else {
			return None
		}
	}
	
	fn show(&self) {
		for i in 0..self.row {
			for j in 0..self.col {
				let value = self.get(i, j).unwrap();
				print!("v[{i},{j}] = {value}\t")
			}
		println!()
		}
		println!()
	}
	
	fn create(r: usize, c: usize) -> Matrix {
		let mut list = Matrix {
			v: Vec::new(),
			row: 0,
			col: 0,
		};
		
		for _i in 0.. (r * c) {
			list.v.push(0.0)
		}
		list.row = r;
		list.col = c;
		list
	}
	
}

fn main() {
	
	let mut list = Matrix::from_file("/home/andy/my_rust/matrix/data/data2.csv");
	
	list.show();
	
	list.set(5, 1, 500.78);
	
	list.show();
	
	let mut list2 = Matrix::create(3, 5);
	list2.set(0, 0, 1.0);
	list2.set(1, 1, 1.0);
	list2.set(2, 2, 1.0);
	
	list2.show();
	
	match list.get(100,100) {
		Some(x) => println!("{}", x),
		None => println!("Выход за пределы массива"),
	}
	
	match list.get(7,3) {
		Some(x) => println!("{}", x),
		None => println!("Выход за пределы массива"),
	}

}
