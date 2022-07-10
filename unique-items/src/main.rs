fn unique(a: &mut Vec<i32>)-> &mut Vec<i32>{
    let mut index=0;
    let mut temp;
    let b = a.clone();
   // let mut remove_elements = Vec::new();
    for i in b{
        temp = i;
        println!("temp {:?}", temp);

        let j = a.iter().skip(index).position(|&x| x == temp).unwrap();
        println!("j {}",j);
        a.remove(j);

        index = index +1;
    }
println!("{:?}", a);
    return a;
}


fn main(){
    let mut input: Vec<i32> = vec![2,1,1,1,4];
    let answer: &mut Vec<i32> = unique(&mut input);
    println!("unique items -> {:?}", answer)
}

