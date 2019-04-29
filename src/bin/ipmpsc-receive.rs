#![deny(warnings)]

use failure::Error;

fn main() -> Result<(), Error> {
    let (name, rx) = ipmpsc::channel::<String>(1024)?;

    println!("name is {}", name);

    loop {
        println!("got {:?}", rx.recv()?);
    }
}
