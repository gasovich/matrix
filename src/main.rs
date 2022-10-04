use std::fs::File;
use std::io::{Read, Write};

struct Matrix {
	v: Vec<f64>,
	row: usize,
	col: usize,
}

impl Matrix {
	fn from_file(path: String) -> Matrix { // Чтение матрицы из файла
		let mut list = Matrix {
			v: Vec::new(),
			row: 0,
			col: 0,
		};
		
		println!("-->{}", path);
		
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
		println!("row = {}\tcol = {}", list.row, list.col);
		list
	}
	
	fn get(&self, i: usize, j: usize) -> Option<f64> { // Получить элемент матрицы по индексу
		if i < self.row && j < self.col {
			return Some(self.v[i * self.col + j])
		} else {
			return None
		}
	}

	fn set(&mut self, i: usize, j: usize, x: f64) -> Option<f64> { // Записать значение элемента по индексу
		if i < self.row && j < self.col {
			self.v[i * self.col + j] = x;
			Some(x)
		} else {
			return None
		}
	}
	
	fn show(&self) { // Вывести матрицу на консоль
		for i in 0..self.row {
			for j in 0..self.col {
				let value = self.get(i, j).unwrap();
				print!("v[{i},{j}] = {value}\t")
			}
		println!()
		}
		println!()
	}
	
	fn create(r: usize, c: usize) -> Matrix { // Создать нулевую матрицу
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
	
	fn mul_by_num(&self, num: f64) -> Matrix { // Умножение матрицы на число
		let mut list = Matrix {
			v: Vec::new(),
			row: self.row,
			col: self.col,
		};
		
		for i in 0..(self.row * self.col) {
			list.v.push(self.v[i] * num)
		}
		list
	}
	
	fn transpose(self) -> Matrix { // Транспонирование матрицы
		let mut list = Matrix::create(self.col, self.row);
		for i in 0..self.row {
			for j in 0.. self.col {
				list.set(j, i, self.get(i, j).unwrap());
			}
		};
		list
	}
	
	fn multipl(&self, x: &Matrix) -> Option<Matrix> { // Умножение матриц
		// self - первый множитель, x - второй множитель
		if self.col != x.row { // Если матрицы не соразмерны, то возвращаем None
			return None
		}
		
		let mut product = Matrix::create(self.row, x.col); // Матрица результата
		
		for k in 0..x.col {
			for i in 0..self.row {
				let mut sum = 0.0;
				for j in 0..self.col {
					let a = self.get(i, j).expect("Out of bound!"); 
					let b = x.get(j, k).expect("Out of bound!");
					sum += a * b
				}
				product.set(i, k, sum);
			}
		}
	
		return Some(product)
	}

	fn save_as(&self, path: String) { // Сохранение матрицаы в файл в формате .csv
		let mut buffer = String::new();
		
		for i in 0..self.row {
			for j in 0..self.col {
				buffer.push_str(&self.get(i, j).unwrap().to_string());
				buffer.push(';')
			}
			buffer.pop();
			buffer.push('\n')
		}
		
		let mut f = File::create(path).expect("Error create file");
		f.write_all(buffer.as_bytes()).expect("Error writing to file");
	}	
	
}

fn main() {
	let base = "/home/andy/my_rust/matrix/data/";
	let name1 = "matr1.csv";
	let name3 = "matr3.csv";
	
	let matr1 = Matrix::from_file(base.to_string() + name1);
	matr1.show();
	let matr3 = matr1.transpose();
	matr3.save_as(base.to_string() + name3);
	matr3.show();
}
