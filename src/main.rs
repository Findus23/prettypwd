extern crate core;

use std::env;


fn simplify_string(pwd: String, home: String) -> String {
    let prettyhome = pwd.replacen(home.as_str(), "/~", 1);
    let dirs: Vec<&str> = prettyhome.split("/").collect();
    let mut print_dirs: Vec<&str> = Vec::new();
    if dirs.len() > 4 {
        if dirs[1] != "~" {
            print_dirs.push("")
        }
        print_dirs.push(dirs[1]);
        print_dirs.push("…");
        print_dirs.extend(dirs[dirs.len() - 2..].to_vec())
    } else { print_dirs = dirs }
    // println!("{:#?}", print_dirs);
    return print_dirs.join("/").replacen("/~", "~", 1);
}

fn main() {
    let pwd = env::var("PWD");
    let pwd = match pwd {
        Ok(test) => test,
        Err(error) => panic!("{:?}", error)
    };
    let home = env::var("HOME");
    let home = match home {
        Ok(test) => test,
        Err(error) => panic!("{:?}", error)
    };
    println!("{}", simplify_string(pwd, home));
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_HOME: &str = "/home/user";

    fn comparison_helper(input: &str, output: &str) {
        assert_eq!(
            simplify_string(input.to_string(), TEST_HOME.to_string())
            , output.to_string()
        );
    }

    #[test]
    fn it_works() {
        comparison_helper("/test", "/test");
    }

    #[test]
    fn shortening() {
        comparison_helper("/var/log/sometool", "/var/log/sometool");
        comparison_helper("/var/log/sometool/subdir", "/var/…/sometool/subdir");
    }

    #[test]
    fn homedir() {
        comparison_helper("/home/user/sometool/subdir", "~/sometool/subdir");
        comparison_helper("/home/user/sometool/subdir/subsubdir", "~/…/subdir/subsubdir");
    }
}
