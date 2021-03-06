//在这个文件中，是关于trait的基础部分。
//主要是三部分：
//1.trait的声明
//2.需要在结构体中实现的trait的方法体
//3.无需在结构体中实现的trait默认方法
use about_trait::custom;

/// 声明一个特性(trait)
/// 在接口里定义一个默认方法
/// 为了在其他文件中访问trait_demo必须以pub开头
pub trait trait_demo {
    // fn中不加pub
    /// 定义了一个方法的结构体，具体的实现要到具体struct的impl中进行
    fn show(&self)->String;
    /// 定义了一个默认方法，这个方法无需去具体的struct中实现
    fn show_default(&self)
    {
        let res=self.show();
        println!("这是从show_default中的调用结果=>\n{}",res);
    }
}


///苹果结构，里面包含了苹果的名字，尝起来的味道。
pub struct Apples {
    ap_name:String,
    ap_taste:String,
}
///特性trait_demo在apples中的实现，实现方法show
impl trait_demo for Apples {
    fn show(&self)->String
    {
        format!("苹果品种:{},尝起来是{}的",self.ap_name,self.ap_taste)
    }
}
///香蕉结构，包含了香蕉的颜色，产地，售价
pub struct Banana
{
    b_color:String,
    b_origin:String,
    b_price:String,
}
///特性trait_demo在香蕉类中的实现
impl trait_demo for Banana
{
    fn show(&self)->String
    {
        format!("香蕉，颜色：{}，产地：{}，售价：{}",self.b_color,self.b_origin,self.b_price)
    }
}
/// 这个方法是对所有继承trait_demo特性的实现的调用
/// 其实多少有些类似面向对象里面的重载
/// GetShow方法可以根据item的类型(具体的struct)，决定究竟去实现哪一个struct里的show方法
fn GetShow<T:trait_demo>(item:T)
{
    println!("调用结果：\n{}",item.show())
}

fn GetShow_default<T:trait_demo>(item:T)
{
    item.show_default();
}

pub fn main() {
    let my_apple=Apples
    {
        ap_name:"红富士".to_string(),
        ap_taste:"甜".to_string(),
    };
    println!("\n以下是在trait中定义方法体，在具体的struct中进行方法实现的demo调用");
    GetShow(my_apple);
    println!("\n\n下面这个是在trait中直接定义了方法实现，无需在struct中进行再次定义的demo调用");
    let my_apple=Apples
    {
        ap_name:"国光".to_string(),
        ap_taste:"酸".to_string(),
    };
    GetShow_default(my_apple);
}
///调用custom模块(mod)中的orange
pub fn GetOrange()
{
    
    let ora=custom::Orange{
        o_type:"不知火".to_string(),
        o_market:"沃尔玛".to_string(),
    };
    println!("\n以下为从一个模块中实现另一个模块的特性的demo");
    GetShow_default(ora);
}


