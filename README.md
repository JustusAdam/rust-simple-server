# simple-server

In order to acquaint myself with the [rust][] programming language I am implementing a very simple web server. It'll simply serve documents relative to the directory it was invoked in.

[rust]: https://www.rust-lang.org

## Installation  advice

- Clone the repository
- Install using cargo

### Installing `openssl` on some OSX Systems

Sometimes cargo won't find the openssl headers and libraries. I finally got it to install when trying these flags

    OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
    OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib
    LDFLAGS=-L/usr/local/opt/openssl/lib
    CFLAGS=-I/usr/local/opt/openssl/include

whilst compiling the dependencies.
