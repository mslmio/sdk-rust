use mslm::Client;
use mslm::otp::OtpTokenVerifyReq;

#[tokio::main]
async fn main() {
    let c = Client::init("<>");

    let otp_token_verify_req = OtpTokenVerifyReq {
        phone: String::from("<>"),
        token: String::from("<>"),
        consume: Some(true),
    };

    let res = c.otp.verify(&otp_token_verify_req, None).await;

    match res {
        Ok(response) => {
            println!("Response:\n{:#?}", response);
        }

        Err(err) => {
            eprintln!("Error: {:#?}", err);
        }
    }
}
