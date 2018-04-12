extern crate rustDemo;

use rustDemo::about_trait;
use std::io;
fn main() {
    let tips="\n\n>>>>>>>>>>>>>>以下是和demo有关的小demo，键入对应的值，可以看到运行后的效果<<<<<<<<<<<<<<<<<\n
====================================================\n
1==>关于trait(特性)的例子。源码路径：src/about_trait/\n
0==>打断程序\n";
    while true {
        println!("{}",tips);
        let mut ipt=String::new();
        io::stdin().read_line(&mut ipt).unwrap();
        let my_ipt:&str=ipt.trim_matches('\n');
        
        /// match用于匹配
        /// 用法：
        /// ```
        /// match x
        /// {
        ///    case1=>情况1的行为,
        ///    case2=>情况2的行为,
        ///    _=>其余所有情况的行为,
        /// }
        /// ```
        /// 需要注意的是，match里面，应该列举尽所有的可能，所以一般得有_这个符号
        /// 如果匹配成功后要执行批量语句，可使用{}：`case1=>{}`。如下方的0方法
        match my_ipt {
            "0" => {
                println!("输入0的时候，我们做了一个多操作。");
                panic!("panic宏可以理解为将程序以报错的方式打断。\n程序被人工打断！")},
            "1" => about_trait::mytrait::main(),
             _  => println!("输入的值非法，请重新输入：\n"),
        }
    }
}
