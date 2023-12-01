# Mslm Rust SDK

The official Rust SDK for Mslm APIs.

## Usage

Add the following to your Cargo.toml file:

```toml
[dependencies]
mslm = "1.0.0"
```

Reference the `mslm` crate in your program and write some code.
Make sure to initialize the Client with your API key and handle the result accordingly.

```rust
use mslm::Client;

#[tokio::main]
async fn main() {
    let c = Client::init("YOUR_API_KEY");
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
```

Sub-client can be initiated directly too:

```rust
use mslm::Client;
use mslm::otp::OtpSendReq;

#[tokio::main]
async fn main() {
    let c = Client::init("");
    let mut otp = c.otp;
    otp.set_api_key("YOUR_OTP_API_KEY");

    let otp_send_req = OtpSendReq {
        phone: String::from("<>"),
        tmpl_sms: String::from("<>"),
        token_len: 6,
        expire_seconds: 60,
    };

    let res = otp.send(&otp_send_req, None).await;

    match res {
        Ok(response) => {
            println!("Response:\n{:#?}", response);
        }

        Err(err) => {
            eprintln!("Error: {:#?}", err);
        }
    }
}
```

## About Mslm

Mslm focuses on producing world-class business solutions. It’s the
bread-and-butter of our business to prioritize quality on everything we touch.
Excellence is a core value that defines our culture from top to bottom.

[![image](https://avatars.githubusercontent.com/u/50307970?s=200&v=4)](https://mslm.io/)