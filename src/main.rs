mod gitwrapper;
use gitwrapper::GitWrapper;

fn main() {
    let mut git_wrapper = GitWrapper::new();
    let (code, out, err) = git_wrapper.exec();
    
    print!("{}", out);
    eprint!("{}", err);
    
    std::process::exit(code);
}




