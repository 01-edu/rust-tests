/*
## unwrap_or_expect

### Instructions

Create a **function** named **fetch_data** which has two arguments:
- `server`: Which is a `Result` having the server url or an error message inside.
- `security_level`: Which is an `enum` defining the behavior of the function in case of errors.

The `security_level` will work as follow:
- `Unknown`: The function panics without printing any custom message.
- `High`: The function panics and prints the error message `ERROR: program stops`.
- `Medium`: The function returns the string `WARNING: check the server`.
- `Low`: The function returns the string `Not found: [SERVER_URL]`.
- `BlockServer`: The function will panic only if the `Result` value is `Ok` and the error message will be the string contained in `Ok`.

To return from **fetch_data** you must use `expect`, `unwrap_or`, `unwrap_err`, `unwrap` and `unwrap_or_else`.

### Expected Functions

```rust
pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {

}
```

### Usage

Here is a program to test your function:

```rust
use unwrap_or_expect::*;

fn main() {
    println!("{}", fetch_data(Ok("server1.com".to_string()), Security::Medium));
    println!("{}", fetch_data(Err(String::new()), Security::Medium));
    println!("{}", fetch_data(Err("server2.com".to_string()), Security::Low));

    // Panics with no custom message
    // fetch_data(Err("ERROR CRITICAL".to_string()), Security::Unknown);

    // Panics with the message "ERROR: program stops"
    // fetch_data(Err(String::new()), Security::High);

    // Panics with the message "malicious_server.com"
    // fetch_data(Ok("malicious_server.com".to_string()), Security::BlockServer);
}
```

And its output:

```console
server1.com
WARNING: check the server
Not found: server2.com
$
```

### Notions

- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Unwrap keywords](https://doc.rust-lang.org/std/?search=unwrap)
*/

pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(),
        Security::High => server.expect("ERROR: program stops"),
        Security::Medium => server.unwrap_or("WARNING: check the server".to_string()),
        Security::Low => server.unwrap_or_else(|url| {
            "Not found: ".to_string() + url.as_str()
        }),
        Security::BlockServer => server.unwrap_err()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_expect() {
        fetch_data(Err(String::new()), Security::High);
    }
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\"")]
    fn test_unwrap() {
        fetch_data(Err("ERROR CRITICAL".to_string()), Security::Unknown);
    }
    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn test_unwrap_err() {
        fetch_data(Ok("malicious_server.com".to_string()), Security::BlockServer);
    }
    #[test]
    fn test_unwrap_or() {
        assert_eq!(
            fetch_data(Err(String::new()), Security::Medium),
            "WARNING: check the server".to_string()
        );
    }
    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(
            fetch_data(Err("another_server.com".to_string()), Security::Low),
            "Not found: another_server.com".to_string()
        );
    }
    #[test]
    fn test_ok() {
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Low),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Medium),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::High),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Unknown),
            "server.com"
        );
    }
}
