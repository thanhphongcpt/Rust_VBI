//Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
//Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//            let sub_arr = [6,8,10];
use std::{array, io};

fn main() {
    let  arr_big: [i32;4] = [3,2,4,9 ];
    let  arr_sm = [4,9]; 

    let b_len = arr_big.len();
    let s_len = arr_sm.len();
    let mut same = 0 ; 
    let mut start_pos = 0;

    for i in 0..s_len{

        for j in start_pos.. b_len{
            if arr_sm[i] == arr_big[j]{
                same +=1;
                start_pos = j;
            }
        }
        if same == s_len{
            println!("same");
            break;
        }
    }
    if same != s_len{
        println!("false");
    }
   
    /*------------------------------------------------------------------------ */
    //bai 2
    /*------------------------------------------------------------------------ */
//input tu ban phim
    let og_ =
           "This is a regular paragraph with the default style of Normal. 
            This is a regular paragraph with the default style of Normal. 
            This is a regular paragraph with the default style of Normal.
            This is a regular paragraph with the default style of Normal. 
            This is a regular paragraph with the default style of Normal.";

    let mut repeat = 0;
    let mut input_char = String::new();
    println!("Nhap tu ban phim: ");
    io::stdin().read_line(&mut input_char).unwrap();
    input_char.pop();
    let mut _t;
    let mut new_str;

//kiem tra ki tu

    if let Some(p_index) = og_.find(&input_char)
    { 
        new_str = og_.split_at( p_index + input_char.len());
        _t = new_str.1;
        repeat +=1;
        while let Some(c) = _t.find(&input_char){
            new_str = _t.split_at( c + input_char.len()); 
            _t = new_str.1;
            repeat +=1;
        }
        println!("p_index: {}",repeat);

   
    }
    else {
        println!("None");
    }
}fn main
