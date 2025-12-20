use std::{fs, io};

trait GetData{
    fn get_data(&self)->Result<String,WriteError>;
}

struct TextSource {
    data: String
}

struct FileSource{
    file:String
}

impl GetData for TextSource{
    fn get_data(&self)->Result<String,WriteError> {
        Ok(self.data.to_owned())
    }
}

impl GetData for FileSource{
    fn get_data(&self)->Result<String,WriteError> {
        match fs::read_to_string(&self.file){
            Ok(data)=>Ok(data),
            Err(e)=> Err(WriteError::Io(e))
        }
    }
}


trait WriteData{
    fn write_data(&self,data:&str)->Result<(),WriteError>;
}

struct ConsoleWriter;
struct FileWriter{
    path:String
}

impl WriteData for ConsoleWriter{
    fn write_data(&self,data:&str)->Result<(),WriteError>{
        println!("Data is {}",data);
        Ok(())
    }
}

impl WriteData for FileWriter{
    fn write_data(&self,data:&str)->Result<(),WriteError>{
        match fs::write(&self.path, data){
            Ok(data)=> Ok(()),
            Err(e)=>Err(WriteError::Io(e))
        }
    }
}


fn main()->Result<(),WriteError>{

    let s1 = TextSource{
        data: "Hello My name is Jayant".to_string()
    };

    let s2 = FileSource{
        file: "./data.txt".to_string()
    };

    let sources:Vec<&dyn GetData> = vec![
        &s1,&s2
    ];

    let write_type_1 = ConsoleWriter;
    let write_type_2 = FileWriter{
        path:"output.txt".to_string()
    };

    for source in sources {
        let content = source.get_data()?;
        println!("content is {:?}",content);
        write_type_1.write_data(&content)?;
        write_type_2.write_data(&content)?;
    }

    Ok(())
}

#[derive(Debug)]
enum WriteError{
    Io(std::io::Error),
    // There can be multiple here like sqlx error , S3 error
}