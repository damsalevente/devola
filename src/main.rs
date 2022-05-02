use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use rppal::uart::{Parity, Uart, Error};

/* hall write -> detect -> hall command: read  */

enum Commands{
    HallRequest,
    HallSend,
    PhaseRequest,
    PhaseSend
}

async fn serve_devola(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match(req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try posting data to /echo");
        },
        (&Method::POST, "/echo") => {
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let mut uart = Uart::with_path("/dev/ttyS1", 115_200, Parity::None, 8, 1).expect("COULD NOT SET UART");
    uart.set_read_mode(1, std::time::Duration::default()).expect("COULD NOT SET UART MODE");
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(serve_devola))
    });
    let mut bytes_read: usize;
    let mut buffer = [0u8; 10];
    loop{
        bytes_read = uart.read(&mut buffer).expect("unable to read");
        if(bytes_read !=0)
        {
            println!("Recieved: {:?}", buffer);
        }

    }
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await { 
        eprintln!("server error");
    }
}
