use std::marker::Copy;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::PartialEq;
use std::ops::MulAssign;
use std::ops::Mul;
use std::ops::AddAssign;
use std::convert::TryInto;


pub struct Matrix<T> {
    data: Box<[T]>,
    row: usize,
    col: usize,
  }

impl <T:Copy + Display + 
        Add<Output = T> + Sub<Output = T> + 
        PartialEq + MulAssign + Mul<Output = T> + 
        AddAssign> 
                   Matrix<T> {
    // конструктор
    pub fn new(val:T, row: usize, col: usize) -> Self {
      
      assert!(
        row > 0 && col > 0,
        "Invalid size row or col ({}, {})",
        row, col,
      );

      let data = vec![val; row * col].into_boxed_slice();
      Self { data, row, col }
    }

    // размерность матрицы в кортеже
    pub fn len(&self) -> (usize, usize) {
      (self.row, self.col)
    }
  
    // Вспомогательный метод для расчёта индекса (номер строки, номер столбца)
    fn index(&self, pos_row: usize, pos_col: usize) -> usize {
      assert!(
        pos_row < self.row && pos_col < self.col,
        "Invalid index ({}, {}) for length ({}, {})",
        pos_row, pos_col, self.row, self.col,
      );
      pos_row * self.col + pos_col
    }
  
    // геттер
    pub fn get(&self, pos_row: usize, pos_col: usize) -> T {
      self.data[self.index(pos_row, pos_col)]
    }

    // сеттер
    pub fn set(&mut self, pos_row: usize, pos_col: usize, value: T) {
      self.data[self.index(pos_row, pos_col)] = value;
    }

    // распечатка матрицы в консоль
    pub fn print_matrix(&self) {
      for i in 0..self.row {
          for j in 0..self.col {
              print!("{} ", self.get(i, j));
              if j == self.col - 1 {
                  println!();
              }
          }
      }
    }

    // сравнение матриц
    pub fn equals(&self, other: &Matrix<T>) -> bool {
      if self.row == other.row && self.col == other.col {
        for i in 0..self.row {
          for j in 0..self.col {
            if self.get(i, j) != other.get(i, j)  {
              return false;
            } 
          }
        }
        true
      } else {
        false
      }
    }

    // умножение матрицы на число
    pub fn mul_matrix_by_value(&mut self, value: T) {
      for i in 0..self.row {
          for j in 0..self.col {
              self.set(i, j, self.get(i,j) * value);
          }
      }
    }

    // транспонирование
    pub fn transpose(&mut self) {
      let mut tmp = Matrix::<T>::new(self.get(0,0), self.col, self.row);

      for i in 0..self.row {
        for j in 0..self.col {
            tmp.set(j, i, self.get(i, j));
        }
      }
      *self = tmp;
    }

  // сокращение матрицы на заданное число строк и столбцов
  pub fn reduse_matrix(&self, rows_reduced: usize, cols_reduced: usize) -> Matrix<T> {
    let mut tmp = Matrix::<T>::new(self.get(0,0), self.row - 1, self.col - 1);
    let mut new_rows: usize = 0;
    let mut new_cols: usize = 0;
    for i in 0..self.row {
      if i != rows_reduced {
        for j in 0..self.row {
          if j != cols_reduced {
            tmp.set(new_rows, new_cols, self.get(i, j));
            new_cols += 1;
          }
        }
        new_rows += 1;
        new_cols = 0;

      }
    }
    return tmp;
  }

  // определитель
  pub fn determinant(&self) -> f64 where f64: From<T> {
    let mut det:f64 = 0.0;

    assert!(
      self.row == self.col,
      "matris not square (row = {}, col = {})",
      self.row, self.col,
    );

    if self.row == 1 {
      det = f64::from(self.get(0,0));
    } else if self.row == 2 {
      det = f64::from(self.get(0,0) * self.get(1, 1)
                       - self.get(0, 1) * self.get(1, 0));               
    } else {
      for i in 0..self.row {
        let buf = self.reduse_matrix(i, 0);
        
        let tmp_det = buf.determinant();
        let tmp_matrix_num = f64::from(self.get(i, 0));
        let tmp_pow: f64 = i32::pow(-1, i.try_into().unwrap()).into();
        det += tmp_matrix_num * tmp_pow * tmp_det;
      }
    }
    return det;
  }

  // получение матрицы алгебраических дополнений
  pub fn calc_complements(&self) -> Matrix<f64>  where f64: From<T> {
    assert!(
      self.row == self.col && self.row > 1,
      "matris not square or size = 1 (row = {}, col = {})",
      self.row, self.col,
    );

    let mut result = Matrix::<f64>::new(0.0, self.row, self.col);
    
    for i in 0..self.row {
      for j in 0..self.col {
        let buf = self.reduse_matrix(i, j);
        
        
        let pow: f64 = i32::pow(-1, (i + j).try_into().unwrap()).into();
        let det = buf.determinant();
        result.set(i, j, det * pow);
      }
  }
  return result;
  }

  // инвертированная матрица
  pub fn inverse_matrix(&self)-> Matrix<f64>  where f64: From<T> {
    let det = self.determinant();
    assert!(
      det != 0.0,
      "determinant == 0",
    );
    let mut result = self.calc_complements();
    result.transpose();
    result.mul_matrix_by_value(1.0 / det);
    return result;
  }
}


// Реализация трэйта add для типа Matrix<T>.
// Здесь переопределяется оператор сложения для матриц.
impl<T:Copy + Display + 
     Add<Output = T> + 
     Sub<Output = T> + 
     PartialEq + MulAssign + 
     Mul<Output = T> + AddAssign> 
                      Add<Matrix<T>> for Matrix<T>
                             where T: Add<Output = T> {
  type Output = Matrix<T>;

  fn add(self, rhs: Matrix<T>) -> Matrix<T> {

    assert!(
      rhs.row == self.row && rhs.col == self.col,
      "Invalid matrix, first matrix = ({}, {}) second matrix = ({}, {})",
       self.row, self.col, rhs.row, rhs.col,
    );

    let mut new_matrix = Matrix::<T>::new(self.get(0,0), self.row, self.col);

    for i in 0..self.row {
      for j in 0..self.col {
        new_matrix.set(i, j, self.get(i, j) + (rhs.get(i, j)));
      }
    }
    return new_matrix;
  }
}

// имплементация трэйта вычитания для типа Matrix<T>, преопределяем оператор вычитания
impl<T:Copy + Display + 
     Add<Output = T> + 
     Sub<Output = T> + 
     PartialEq + MulAssign + 
     Mul<Output = T> + AddAssign>
                      Sub<Matrix<T>> for Matrix<T> 
                              where T: Sub<Output = T> {
  type Output = Matrix<T>;

  fn sub(self, rhs: Matrix<T>) -> Matrix<T> {

    assert!(
      rhs.row == self.row && rhs.col == self.col,
      "Invalid matrix, first matrix = ({}, {}) second matrix = ({}, {})",
       self.row, self.col, rhs.row, rhs.col,
    );

    let mut new_matrix = Matrix::<T>::new(self.get(0,0), self.row, self.col);

    for i in 0..self.row {
      for j in 0..self.col {
        new_matrix.set(i, j, self.get(i, j) - (rhs.get(i, j)));
      }
    }
    return new_matrix;
  }
}

// переопределяем опреатор умножения
impl<T:Copy + Display + 
     Add<Output = T> + 
     Sub<Output = T> + 
     PartialEq + MulAssign + 
     Mul<Output = T> + AddAssign>
                       Mul<Matrix<T>> for Matrix<T> 
                                where T: Mul<Output = T> {
  type Output = Matrix<T>;

  fn mul(self, rhs: Matrix<T>) -> Matrix<T> {

    assert!(
      rhs.col == self.row && rhs.row == self.col,
      "Invalid matrix, first matrix = ({}, {}) second matrix = ({}, {})",
       self.row, self.col, rhs.row, rhs.col,
    );

    let mut new_matrix = Matrix::<T>::new(self.get(0,0), self.row, rhs.col);

    for i in 0..self.row {
      for j in 0..rhs.col {
        for n in 0..rhs.row {
          if n == 0 {
            new_matrix.set(i, j, self.get(i, n) * rhs.get(n, j));
          } else {
            new_matrix.set(i, j, new_matrix.get(i, j) + (self.get(i, n) * rhs.get(n, j)));
          }
        }
      }
    }
    return new_matrix;
  }
}
