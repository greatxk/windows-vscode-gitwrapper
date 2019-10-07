use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

const GIT_EXE_PATH: &'static str = "git.exe";
const CYGPATH_EXE_PATH: &'static str = "cygpath.exe";

pub struct GitWrapper {
    file: Option<File>,
}

impl GitWrapper {
    /// 新建
    pub fn new() -> GitWrapper {
        GitWrapper { file: None }
    }

    /// 执行, 返回执行git的退出码和输出
    pub fn exec(&mut self) -> (i32, String, String) {
        let mut args_iter = env::args();
        args_iter.next().unwrap();

        let args: Vec<String> = args_iter.collect(); // 传入参数列表
        let mut git_args: Vec<String> = Vec::new(); // 转换路径后的参数列表

        for arg in &args {
            let arg = arg.replace("{", "\\{").replace("}", "\\}");
            // 将windows路径转成unix路径
            if arg.find(":/").is_some() || arg.find(":\\").is_some() {
                git_args.push(self.convert_path_from_windows_to_unix(&arg));
            } else {
                git_args.push(arg.to_owned());
            }
        }

        // 判断git调用后是否需要转换调用结果
        let mut need_tranlate_path: bool = false;
        let full_args = args.join(" ");
        if full_args.find("rev-parse --show-toplevel").is_some() {
            need_tranlate_path = true;
        }
        // 调用git
        let (code, mut out, err) = self.call_git(&git_args);

        // 路径转换
        if code == 0 && need_tranlate_path && out.find("/").is_some() {
            // 转换为window路径
            out = self.convert_path_from_unix_to_windows(&out);
            out = out.replace("\\", "/");
            if out.len() >= 2 && &out[1..2] == ":" {
                out = String::from(&out[0..1]).to_lowercase() + &out[1..];
            }
        }
        //self.log(&format!("code={}\n out={:?}\n err={:?}", code, out, err));
        (code, out, err)
    }

    /// 调用git
    fn call_git(&mut self, args: &Vec<String>) -> (i32, String, String) {
        match Command::new(GIT_EXE_PATH).args(args.as_slice()).output() {
            Ok(output) => {
                let mut out = String::new();
                let mut err = String::new();
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    out = stdout;
                }
                if let Ok(stderr) = String::from_utf8(output.stderr) {
                    err = stderr;
                }
                if output.status.code().is_none() {
                    return (-1, out, err);
                }
                (output.status.code().unwrap(), out, err)
            }
            Err(error) => {
                let err = format!("Error:call_git={}", error);
                // 如果出错，则记录到日志文件中
                self.log(&err);
                (-1, "".to_owned(), err)
            }
        }
    }

    /// 将unix路径转换为windows路径
    fn convert_path_from_unix_to_windows(&mut self, path: &str) -> String {
        match Command::new(CYGPATH_EXE_PATH).args(&["-w", path]).output() {
            Ok(output) => {
                let mut out = String::new();
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    out = stdout;
                }

                if out.len() > 0 {
                    return out;
                }
            }
            Err(error) => {
                let err = format!("Error:convert_path_from_unix_to_windows={}", error);
                // 如果出错，则记录到日志文件中
                self.log(&err);
            }
        }
        path.to_owned()
    }

    /// 将windows路径转换为unix路径
    fn convert_path_from_windows_to_unix(&mut self, path: &str) -> String {
        match Command::new(CYGPATH_EXE_PATH).args(&["-u", path]).output() {
            Ok(output) => {
                let mut out = String::new();
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    out = stdout;
                }

                if out.len() > 0 {
                    return out;
                }
            }
            Err(error) => {
                let err = format!("Error:convert_path_from_windows_to_unix={}", error);
                // 如果出错，则记录到日志文件中
                self.log(&err);
            }
        }
        path.to_owned()
    }

    // 记录日志
    fn log(&mut self, log: &str) {
        let mut path = std::env::current_exe().unwrap_or(std::path::PathBuf::from("./"));
        path.pop();
        path.push("windows-vscode-gitwrapper.log");
        // 生成带时间戳的日志
        let date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let content: String = format!("[{}]:{}\r\n", date_time, log);
        if self.file.is_none() {
            self.file = Some(
                OpenOptions::new()
                    .create(true)
                    .truncate(false)
                    .append(true)
                    .open(path)
                    .unwrap(),
            );
        }
        self.file
            .as_mut()
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
        self.file.as_mut().unwrap().flush().unwrap();
    }
}
