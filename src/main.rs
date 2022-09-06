use std::env;



fn max(x:i32,y:i32) -> i32 {
	if x > y{
		return x;
	}
	else{
		return y;
	}
}



fn is_prime(x:i32) -> bool {
	let mut d = 2;
	while d * d <= x{
		if x % d == 0 {
			return false;
		};
		d += 1;
	}
	return true;
}

fn get_multpliers(x:i128) -> Vec<i128> {
     let mut d = 2;
     let mut y = x;
     let mut vec: Vec<i128> = Vec::new();
     while d * d <= y{
     	if y % d == 0 {	
     	     		 vec.push(d); 
	
     		 y  = y / d;
     	}else{
     	d += 1}	
     }
     if y > 1 {vec.push(y)};
     return vec;


}


fn n_primes(n: i32) -> Vec<i32> {
     let mut vec = Vec::with_capacity(n.try_into().unwrap());
     let mut  counter = 1;
     while counter <= n{
     	if is_prime(counter){vec.push(counter)}
     	else {println!("{} is not prime",counter)}
     	counter += 1;
     }
     return vec;
 }


fn main() { 
   
   println!("Hello ALem");
   println!("{} is my age",22);
   let x = "Kutlug";
   let leng = x.len();
   println!("{} length is {}",x,leng);
   let y : i64 = 123;
   println!("{}",y);
   // tuple type 
   let tup = ("12312","what",123);
   let (x,y,z) = tup;
   println!("{},{}",x,tup.1);
   //array type

   let array = [2,3,5,7,11,13];
   let another_primes : [i64;5] = [13,17,19,23,39];
   println!("{}", another_primes[0]);

   let num1 = 101212;
   let num2 = 12312312;
   let what = max(num1,num2);
   println!("maximum is  {} ",what);
   
   let  counter:i32 = 0;
    let numer3 = is_prime(num2);
    println!("{} is_prime = {} ", num2,numer3);
    let args: Vec<_> = env::args().collect();
    //let r = n_primes();
    let rr = get_multpliers(args[1].parse::<i128>().unwrap());
   
    for i in &rr {
    	println!("{}",i);
    }  
}