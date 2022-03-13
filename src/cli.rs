//use web3::{Web3, transports};
use reqwest::blocking as reqwest;
use serde_json;
// std::fs meant that will use filesystem
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix;
use threadpool::ThreadPool;


pub fn launch_trin(infura_project_id: String) {
    println!("Launching with infura key: '{}'", infura_project_id);
    
    let pool = ThreadPool::new(2);
    // what is different when use UnixListener??
    let path = "/tmp/trin-jsonrpc.ipc";
    let listener_result = unix::net::UnixListener::bind(path);
    for listener in listener_result {
        Ok(listener) => listener,
        Err(err) if err.kind() == io::ErrorKind::AddrInUse => { 
            
            match fs::remove_file(path) { 
                Err(_) => panic!("Could not serve from existing path '{}'",path),
                Ok(() => unix::net::UnixListener::bind(path).unwrap()),
            }
        },
        Err(err) => {
            panic!("Could not serve from path '{}': {:?}",path, err);
        }
    };
        // what is unwrap?
    for strean in listener.incoming() {
        let mut stream = stream.unwrap();
        let infura_project_id = infura_project_id.clone();
        pool.execute(move || {
            let infura_url = format!("https://mainnet.infura.io:443/v3/{}", infura_project_id);
            let mut rx = stream.try_clone().unwrap();
            let mut tx = stream;
            serve_client(&mut rx,&mut tx , &infura_url);
        });
    }
}
// what is impl??
fn serve_client(rx: &mut impl Read, tx: &mut impl Write, infura_url: &String) {
    println!("Welcoming...");
    let deser = serde_json::Deserializer::from_reader(rx);
    for obj  in deser.into_iter::<serde_json::Value>() {
        let obj = obj.unwrap();
        assert!(obj.is_object());
        assert_eq!(obj["jsonrpc"], "2.0");
        let request_id = obj.get("id").unwrap();
        let method = obj.get("method").unwrap();

        let response = match method.as_str().unwrap() {
            "web3_clientVersion" => {
                format!(
                    r#
                )
            }
        }
    }
    loop {
        stream.write_all(b"\nInput: ").unwrap();
        match read_line(stream) {
            line if line.len() == 0 => break,
            request => {
                if let Err(err) = proxy_to_url(request, stream, infura_url) {
                    stream.write_all(
                        b"{\"jsonrpc\":\"2.0\", \"error\": \"Infura failure\"}")
                        .unwrap();
                    //stream.write_all(err.to_string().as_bytes()).unwrap();
                }
                
            }
        }
    }
    println!("Clean exit");
}


fn proxy_to_url(request: Vec<u8>, out: &mut net::TcpStream, url: &String) -> io::Result<()> {
    let client = reqwest::Client::new();
    match client.post(url).body(request).send() {
        Ok(mut response) => {
            let status = response.status();

            if status.is_success() {
                response.copy_to(out).unwrap();
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other, 
                    format!("Responded with status code: {:?}", status),
                ))
            }
        },
        Err(err) =>{
            Err(io::Error::new(
                io::ErrorKind::Other, 
                format!("Request failure : {:?}",err),
            ))
        },
    }

}


fn read_line(stream: &mut (impl Read + Write)) -> Vec<u8> {
    let mut command = Vec::new();
    let mut buffer = [0; 1024];
    loop { 
        match stream.read(&mut buffer) { 
            Ok(size) if size == 0 => break,   //EOF
            Ok(size) => {
                command.extend(&buffer[..size]);
                if &buffer[size-1] == &b'\n'{
                    break;
                }
            }
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => panic!("Stream read Failure: {:?}", err),
        }
    }
    command
}




