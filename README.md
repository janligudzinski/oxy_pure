# oxy_pure
![Build status](https://travis-ci.org/oreganoli/oxy_pure.svg?branch=master)

An automatic spam deleter for o2.pl email accounts running on AWS Lambda.
## Motivation
The Polish mail provider o2 is funded by ads. These ads arrive directly in a user's inbox. Unlike the targeted ads enabled by Google or Facebook's algorithms,
they are usually completely irrelevant to the user's interests and arrive in the wee hours of the night, which is especially enjoyable if one sleeps next to their phone.
Most users don't read them, but they still generate unsightly "unread" notifications and clutter.

The server-side filtering o2 provides makes an exception for these spam e-mails and will not delete them or mark them as read, for obvious reasons. While mail clients such as Thunderbird do provide an option to filter mail client-side, no mainstream mobile mail client exists that provides filtering.

Due to various reasons, switching providers might not always be an option.

Thus, oxy_pure was born, initially as a .NET Core application based on MailKit. While effective, it required a constantly-running server and was prone to crashing if o2 returned a bad response for whatever reason.
Therefore, I have rewritten oxy_pure as an AWS Lambda function that only runs - and costs money - at chosen times.
## Compilation and usage
Install and enable Docker to make use of the Rust-for-musl image. Install [`just`](https://github.com/casey/just/)
Build for a `musl` target with the provided `build-release` command in the Justfile, zip up the resulting executable and upload to an AWS Lambda function.

Set the `O2_USERNAME` environment variable to your o2 username (unlike with Gmail accounts, without the "@o2.pl" suffix), set `O2_PASSWORD` and, if you want verbose logs,
set `RUST_LOG` to `INFO`. 

Set a CloudWatch timer event to trigger the function every few minutes. In the author's experience, the function takes on average 4.5 seconds to execute.
