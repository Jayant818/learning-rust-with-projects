
pub struct LogEntry<'a>{
    message:&'a str,
    date:&'a str,
}

fn main() {
    // Let say it is stored in a HEAP;
    let log_data = "2025-12-14 [INFO] System boot initialized\n\
                    2025-12-14 [ERROR] Database connection failed: Timeout\n\
                    2025-12-14 [ERROR] Critical failure: Disk full".to_string();


    // Now we have to parse the error
    // as we are making it zero_copy so we are only storing the reference
    let res = error_parse(&log_data);

    for entry in res{
        println!("Found Error \n Date: {} \n Message: {}",entry.date,entry.message);
    }
}


pub fn error_parse<'a>(log_data:&'a str)->Vec<LogEntry<'a>>{
    let mut res = Vec::new();

    for lines in log_data.lines(){
        if let Some((date,message)) = lines.split_once(" [ERROR] "){
            res.push(
                LogEntry{
                    date,
                    message
                }
            );
        }
    }
    res
}
