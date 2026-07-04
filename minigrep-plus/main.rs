use std::env;//collect
use std::fs;//read_to_string
use std::process;
use minigrep::Config;
fn main(){
    //let args: Vec<String> = env::args().collect();
    //获取命令行参数

   let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}",err);
        std::process::exit(1);
    });//错误返回此匿名函数，把没用的信息删除

    if let  Err(e)=minigrep::run(config){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}

