// macro_rules! create_struct{
//     ($StrucName:n,$key:k,&value:v)=>{
//         $StrucName=$StrucName
//         {
//             $key:$value
//         }
//     }
// };
//valid fragment specifiers are `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `path`, `meta`, `tt`, `item` and `vis`

macro_rules! create_fns {
    ($fn:ident) => (fn $fn(){
        println!("由宏创建！");
    })
}

struct apple
{
    name:String
}

impl  apple {
    fn show(&self)
    {
        println!("我是一个削苹果，我的名字叫：{}",self.name);
    }
    // add code here
}

pub fn main() {
    create_fns!(apple);
    apple();
    
}