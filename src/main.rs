use serde::{Deserialize, Serialize};

mod types;

fn main() -> Result<(), anyhow::Error> {
    let client = reqwest::blocking::Client::new();

    // find first 100 prime numbers
    let mut output = Vec::new();
    // the number of prime numbers we have found
    let mut count = 0;
    // the number we are currently checking
    let mut num = 0;
    while count < 100 {
        if is_prime(num) {
            output.push(num);
            count += 1;
        }
        num += 1;
    }

    let mut echo_output: Vec<Body> = Vec::new();

    // http echo
    for prime in output.iter() {
        let body = Body::new(*prime);
        let res = client
            .post("https://echo.free.beeceptor.com")
            .json(&body)
            .send()?
            .json::<types::Root<Body>>()?;
        echo_output.push(res.parsed_body);
    }

    for (i, prime) in output.iter().enumerate() {
        println!("{}: {}", i + 1, prime);
    }

    Ok(())
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Body {
    prime: u32,
}

impl Body {
    fn new(prime: u32) -> Self {
        Self { prime }
    }
}
