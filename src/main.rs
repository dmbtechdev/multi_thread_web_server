use multi_thread_web_server::{ThreadPool,Colorize,handle_connection};

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut init = true;
    loop {
        if init {println!("{}","Server started".green());init = !init;} else {println!("{}","Restarting".green());}

        let pool = ThreadPool::new(10);
        // let mut count = 0;
        // println!("{:?}",listener.local_addr());

        for stream in listener.incoming().take(9) {
            // count +=1;
            // println!("Connection established! count: {:?}",count );
            // println!("Stream {:?}",stream);
            
            let stream = stream.unwrap();
            // handle_connection(stream);
            // match count {
            //     1..=4 => {
            //         thread::spawn(|| {
            //             handle_connection(stream);
            //             //     pool.execute(move || {
            //             // handle_connection(stream);
            //         });
            //     }
            //     _ => {
            //         println!("Connections: {count}\n Too many connections! Disconnecting.");
            //         stream.shutdown(std::net::Shutdown::Both).unwrap();
            //     }
            // }
            // thread::spawn(|| {
            //     handle_connection(stream);
            // });

            // println!("stream: {:?}", stream);
            pool.execute( || {
                handle_connection(stream);
            });
        }        
    }
}
