//extern crate targetFile;

use about_trait::mytrait::trait_demo;


pub struct orange {
    pub o_type: String,
    pub o_market:String,
}
impl trait_demo for orange {
    fn show(&self)->String
    {
        println!("这是在另一个mod中实现strait_demo特性的结构体");
        format!("橘子品种：{},购买超市：{}",self.o_type,self.o_market)
    }
}
