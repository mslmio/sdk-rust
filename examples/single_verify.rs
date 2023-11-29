use mslm::Client;

#[tokio::main]
async fn main() {
    let c = Client::init("");
    let res = c.email_verify.single_verify("support@mslm.io", None).await;

    match res {
        Ok(response) => {
            println!("Response:\n{:#?}", response);
        }

        Err(err) => {
            eprintln!("Error verifying email: {:#?}", err);
        }
    }
}
