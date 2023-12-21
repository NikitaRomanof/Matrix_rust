pub mod matrix;

fn main() {
     println!("______create matrix by new method_____");
     let mut load_matrix = matrix::Matrix::<f32>::new(5.5, 3, 3);
     println!("load_matrix: ");
     load_matrix.set(2, 0, 2.0);
     load_matrix.print_matrix();
     println!("row = {}, col = {}", load_matrix.len().0, load_matrix.len().1);
     println!("*****END*****");

     println!("______sum matrix_____");
     let first_sum = matrix::Matrix::<f32>::new(2.0, 3, 3);
     let second_sum = matrix::Matrix::<f32>::new(3.0, 3, 3);
     let rez_sum = first_sum + second_sum;
     rez_sum.print_matrix();
     println!("*****END*****");

     println!("______sub matrix_____");
     let first_sub = matrix::Matrix::<f32>::new(7.0, 3, 3);
     let second_sub = matrix::Matrix::<f32>::new(3.0, 3, 3);
     let rez_sub = first_sub - second_sub;
     rez_sub.print_matrix();
     println!("*****END*****");

     println!("______mul matrix_____");
     let first_mul = matrix::Matrix::<f32>::new(4.0, 2, 3);
     let second_mul = matrix::Matrix::<f32>::new(5.0, 3, 2);
     let rez_mul = first_mul * second_mul;
     rez_mul.print_matrix();
     println!("*****END*****");

     println!("______equals matrix_____");
     let first_eq = matrix::Matrix::<f32>::new(2.0, 2, 2);
     let second_eq = matrix::Matrix::<f32>::new(2.0, 2, 2);
     println!("equals = {}", first_eq.equals(&second_eq));
     println!("*****END*****");

     println!("______mul matrix by value_____");
     let mut first_mul_val = matrix::Matrix::<f32>::new(4.0, 2, 2);
     first_mul_val.mul_matrix_by_value(2.0);
     first_mul_val.print_matrix();
     println!("*****END*****");

     println!("______transpose matrix_____");
     let mut tr = matrix::Matrix::<f32>::new(4.0, 3, 2);
     println!("BEFORE");
     tr.print_matrix();
     tr.transpose();
     println!("row = {}, col = {}", tr.len().0, tr.len().1);
     println!("AFTER");
     tr.print_matrix();
     println!("*****END*****");

     println!("______reduse matrix_____");
     let red = matrix::Matrix::<f32>::new(4.0, 4, 4);
     let red_2 = red.reduse_matrix(1, 1);
     red_2.print_matrix();
     println!("*****END*****");

     println!("______determinant matrix_____");
     let mut det_matrix = matrix::Matrix::<f32>::new(4.0, 3, 3);
     det_matrix.set(0,0,5.0);
     det_matrix.set(2,2,5.0);

     let det = det_matrix.determinant();
     println!("det = {}", det);
     println!("*****END*****");

     println!("______calc_complements matrix_____");
     let cal_com_m = det_matrix.calc_complements();
     cal_com_m.print_matrix();
     println!("*****END*****");

     println!("______inverse matrix_____");
     let inv = det_matrix.inverse_matrix();
     inv.print_matrix();
     println!("*****END*****");
}
