pub struct ErrorFlag{

}

impl ErrorFlag{
    pub fn help(){
        print!("Usage: tinyscanner [OPTIONS] [HOST] [PORT]\n \
        Options:\n \
            -p               Range of ports to scan\n \
            -u               UDP scan\n \
            -t               TCP scan\n \
            --open           Print opened port only\n \
            --help           Show this message and exit.
        ")
    }
    pub fn erro_option(option: &str){
        println!("active: invalid option -- '{}'\n  \
        Try 'active --help' for more information." ,option)
    }
}