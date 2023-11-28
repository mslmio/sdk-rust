use mslm::Client;
use mslm::otp::OtpSendReq;

#[tokio::main]
async fn main() {
    let c = Client::init("<>");

    let otp_send_req = OtpSendReq {
        phone: String::from("<>"),
        tmpl_sms: String::from("Testing sdk-rust"),
        token_len: 6,
        expire_seconds: 60,
    };

    let res = c.otp.send(&otp_send_req, None).await;

    match res {
        Ok(response) => {
            println!("Response:\n{}", response);
        }

        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
