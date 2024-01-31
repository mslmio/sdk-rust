# Mslm Rust SDK

The official Rust SDK for Mslm APIs.

## Requirements

Install Rust and Cargo by following [these instructions](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Authentication

Mslm's APIs require an API key. If you don't have one, please read [Authentication](https://mslm.io/docs/api/authentication) to understand how to get an API key before continuing.

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

Subclienst can be initiated directly:

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

## Additional Resources

See the official [API Reference Documentation](https://mslm.io/docs/api) for
details on each API's actual interface, which is implemented by this SDK.

## Contributing

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## Security

See [Security Issue
Notifications](CONTRIBUTING.md#security-issue-notifications) for more
information.

## License

This project is licensed under the [MIT License](LICENSE).

## About Mslm

At Mslm, we're all about making things better. Where others see norm, we see
opportunity.

We build world-class solutions to the toughest problems. Excellence is a core
value that defines our approach from top to bottom.

We're all about creating positive value for ourselves, our partners and our
societies.

We do it by focusing on quality and the long-term, while intelligently
maneuvering the complex realities of day-to-day business.

Our partners share our perspective, and we jointly produce truly world-class
solutions to the toughest problems.

[![image](https://avatars.githubusercontent.com/u/50307970?s=200&v=4)](https://mslm.io/)
