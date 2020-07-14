
fn fib(z:&i32){
     let mut a:i32 = 0;
     let mut b:i32 = 1;
     let mut c:i32 = 0;
     println!("{} \n{}",&a,&b);
     for g in 0..*z-2{
         c = a+b;*
         print!("{} \n",c);
         a=b;
         b=c;
     }
}
fn main(){
     let x:i32 = 10;
     fib(&x)
     }
