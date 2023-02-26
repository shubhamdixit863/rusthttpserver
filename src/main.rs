fn main() {
    let string =String::from("127.0.0.1:8080");
    let stringslice=&string[10..];// gives everything after 10 bytes ,might be a problem with emoji and other ascii string
    dbg!(&string);
    dbg!(stringslice);


    //server.run();
}

struct Server{
    addr: String,
}


impl Server{
    fn new(address :String) -> Self{ // alias for Server
        Self{
            addr:address
        }

    }
 // method
    fn run(self){

    }
}