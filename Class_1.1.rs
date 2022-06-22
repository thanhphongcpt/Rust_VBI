//Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
//Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//            let sub_arr = [6,8,10];


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
   
}
