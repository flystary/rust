use std::io::{self, Write};
use std::fs::File;
use std::fs;
use std::path::Path;
use std::time::{SystemTime};
extern crate chrono; 
use chrono::offset::Local; 	// 如果要用UTC时间，可以把Local换成Utc
use chrono::DateTime;
use std::io::prelude::*;
use std::io::BufReader;



enum Command {
    Ls,
    Date,
    Touch(String),
    Cat(String),
    MKdir(String),
    Cp(String, String),
    Mv(String, String),
    Rm(String)
}


impl Command {
    pub fn execute(&self) {
        match self {
            Command::Ls => Command::ls(),
            Command::Date => Command::date(),
            Command::Touch(path) => Command::touch(path.to_string()),
            Command::Cat(path) => Command::cat(path.to_string()),
            Command::MKdir(path) => Command::mkdir(path.to_string()),
            Command::Cp(from, to) => Command::cp(from.to_string(),to.to_string()),
            Command::Mv(from, to) => Command::mv(from.to_string(),to.to_string()),
            Command::Rm(path) => Command::rm(path.to_string()),
        }
    }
    #[allow(dead_code)]
    #[warn(overlapping_range_endpoints)]
    fn ls() {
        let path = Path::new(".");
    if path.is_dir() {
        // 读取目录信息
        match path.read_dir() {
            Ok(de) => {
                // 打印表头
                println!("文件名\t\t文件类型\t权限\t大小\t创建时间\t\t访问时间\t\t修改时间");

                // 遍历目录
                for item in de {
                    match item {
                        Ok(entry) => {
                            // 取得实体属性
                            match entry.metadata() {
                                Ok(metadata) => {
                                    // 文件名
                                    let name = String::from(entry.file_name().to_str().unwrap_or(""));
                                    print!("{}", name);
                                    match name.len() {
                                        0..=9 => print!("\t\t"),
                                        10..=20 => print!("\t"),
                                        _ => print!("\t")
                                    }                             

                                    // 文件类型
                                    if let Ok(ft) = entry.file_type() {
                                        if ft.is_dir() {
                                            print!("目录\t\t");
                                        } else if ft.is_file() {
                                            print!("文件\t\t");
                                        } else if ft.is_symlink() {
                                            print!("符号链接\t\t");
                                        } else {
                                            print!("未知\t\t");
                                        }
                                    } else {
                                        print!("未知\t");
                                    }

                                    // 权限
                                    let p = metadata.permissions();
                                    if p.readonly() {
                                        print!("只读\t", );
                                    } else {
                                        print!("读写\t", );
                                    }                                        

                                    // 大小
                                    print!("{:?}\t", metadata.len());

                                    // 创建时间
                                    let ct = metadata.created().unwrap_or(SystemTime::now());
                                    let dt: DateTime<Local> = ct.into(); 
                                    print!("{}\t", dt.format("%Y-%m-%d %T"));

                                    // 最后访问时间
                                    let ct = metadata.accessed().unwrap_or(SystemTime::now());
                                    let dt: DateTime<Local> = ct.into(); 
                                    print!("{}\t", dt.format("%Y-%m-%d %T"));

                                    // 最后修改时间
                                    let ct = metadata.modified().unwrap_or(SystemTime::now());
                                    let dt: DateTime<Local> = ct.into(); 
                                    print!("{}\t", dt.format("%Y-%m-%d %T"));

                                    // 结束
                                    println!("");
                                },
                                Err(e) => {
                                    println!("取得元数据失败: {:?}", e);
                                },
                            }
                        },
                        Err(e) => {
                            println!("取得文件实体失败: {:?}", e);
                        },
                    }
                }
            },
            Err(e) => {
                println!("读取目录失败: {:?}", e);
            }
        };

    } else {
        println!("{:?} 不是目录", path);
    }

    }

    fn date() {
       let now = Local::now();
       println!("{}", now)
    }

    fn touch(path: String) {
        if path.len() == 0 {
            println!("请输入要创建的文件路径");
            return;
        }

        let p = Path::new(&path);
        

        if p.is_file() || p.is_dir() {
            println!("文件已存在");
            return;
        } 

        match File::create(p) {
            Ok(_) => println!("创建文件成功: {:?}", p),
            Err(e) => println!("创建文件失败: {:?}", e),
        }
    }



    fn cat(path: String) {
       // 检查文件路径是否输入
        if path.len() == 0 {
        println!("请输入要显示的文件路径");
        return;
        }

        match File::open(path) {
            Ok(f) => {
                let mut buf = BufReader::new(f);
                let mut content = String::new();
                while let Ok(size) = buf.read_line(&mut content) {
                    if size > 0 {
                        print!("{}", content);
                        content.clear();        // 每次需要将content清空，否则读取的内容会追加到content结尾
                    } else {
                        println!("");
                        break
                    }
                }
            },
            Err(e) => println!("打开文件出错：{}", e)
        };

    }

    
    fn mkdir(path: String) {
        if path.len() == 0 {
            println!("请输入要创建的目录路径");
            return;
        }

        fs::create_dir_all(path).unwrap_or_else(|e| println!("创建目录失败：{}", e));

    }

    fn rm(path: String) {
        // 检查路径是否输入
        if path.len() == 0 {
            println!("请输入要删除和文件或目录路径");
            return;
        }

        let p = Path::new(&path);
        if p.is_dir() {
            fs::remove_dir_all(p).unwrap_or_else(|e| println!("删除目录失败：{}", e));
        } else if p.is_file() {
            fs::remove_file(p).unwrap_or_else(|e| println!("删除文件失败：{}", e));
        }

    }

    fn cp(from: String, to: String) {
            // 检查路径是否输入
        if from.len() == 0 {
            println!("请输入要复制的文件路径");
            return;
        }

        if to.len() == 0 {
            println!("请输入目标文件路径");
            return;
        }

        fs::copy(from, to).unwrap_or_else(|e| { println!("创建目录失败：{}", e); 0});
    }

    fn mv(from: String, to: String) {
       // 检查路径是否输入
    if from.len() == 0 {
            println!("请输入要复制的文件路径");
            return;
        }

        if to.len() == 0 {
            println!("请输入目标文件路径");
            return;
        }

        fs::rename(from, to).unwrap_or_else(|e| println!("文件改名失败：{}", e));
    }
}

fn get_command(cmd: String) -> Option<Command> {
    let mut inputs: Vec<&str> = cmd.split(' ').collect();

    for _ in 3-inputs.len()..3 {
        inputs.push("");
    }

    match inputs[0] {
        "ls" => Some(Command::Ls),
        "date" => Some(Command::Date),
        "touch" => Some(Command::Touch(inputs[1].to_string())),
        "cat" => Some(Command::Cat(inputs[1].to_string())),
        "mkdir" => Some(Command::MKdir(inputs[1].to_string())),
        "rm" => Some(Command::Rm(inputs[1].to_string())),
        "cp"    =>  Some(Command::Cp(inputs[1].to_string(), inputs[2].to_string())),
        "mv"    =>  Some(Command::Mv(inputs[1].to_string(), inputs[2].to_string())),
        _   => None 
    }

}


fn main() {
    loop {
        print!("cli#>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();
        if cmd == "exit" {
            break;
        }

        match  get_command(cmd.to_string()) {
            Some(cmd) => cmd.execute(),
            None  => println!("正在解锁此命令！！")
        }
    }

}