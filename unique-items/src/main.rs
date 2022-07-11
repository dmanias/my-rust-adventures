fn unique(a: &mut Vec<i32>)-> &mut Vec<i32>{
   // let mut index=0;
    let mut temp=-1;
    // let b = a.clone();
    let mut len = a.len();

   // let mut remove_elements = Vec::new();
    for i in 0 .. len {
        if i<a.len(){
        temp = a[i];
        println!("temp len i {} {} {}", temp, len, i);
        }
        for j in i +1 .. len {
            while j < a.len() && a[j] == temp {
                    println!("a[j] len j {} {} {}", a[j], len, j);
            a.remove(j);
            len = len -1;              
        }
    }     

     //   index = index +1;
    }
println!("{:?}", a);
    return a;
}


fn main(){
    let mut input: Vec<i32> = vec![2,1,1,1,4];
    let answer: &mut Vec<i32> = unique(&mut input);
    println!("unique items -> {:?}", answer)
}

