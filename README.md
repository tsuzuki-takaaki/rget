https://mattgathu.github.io/2017/08/29/writing-cli-app-rust.html

### wget
>Wget is a networking command-line tool that lets you download **files** and interact with REST APIs. It supports the HTTP , HTTPS , FTP , and FTPS internet protocols. Wget can deal with unstable and slow network connections. In the event of a download failure, Wget keeps trying until the entire file has been retrieved.

- progress bar
  - https://docs.rs/indicatif/latest/indicatif/
  - https://github.com/console-rs/indicatif/blob/main/examples/download-continued.rs
- http client
  - hyper: https://crates.io/crates/hyper <- lower level
  - reqwest: https://crates.io/crates/reqwest
  - content_length: may not be returned
    - https://docs.rs/reqwest/0.9.10/reqwest/struct.Response.html#method.content_length
