# `nekoprint`

[![Crates.io](https://img.shields.io/crates/v/nekoprint.svg)](https://crates.io/crates/nekoprint)
[![Docs.rs](https://docs.rs/nekoprint/badge.svg)](https://docs.rs/nekoprint)
[![License](https://img.shields.io/crates/l/nekoprint.svg)](https://github.com/pas2rust/nekoprint/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/nekoprint?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/nekoprint?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/nekoprint?color=orange&logo=Furry%20Network&style=flat&logoColor=white)
![Tests](https://raw.githubusercontent.com/pas2rust/badges/main/nekoprint-tests.svg)
![Crates.io downloads](https://img.shields.io/crates/d/nekoprint.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/pas2rust/nekoprint?color=ff69b4&label=update&logo=git&style=flat&logoColor=white)

**`nekoprint`** is a powerful procedural macro for Rust that automatically generates printing methods for a struct's fields and the entire struct itself. It lets you create fluid, asynchronous logs with different severity levels using a simple, chained syntax.

This library is ideal for **debugging**, **monitoring**, and **diagnostics** in Rust applications, especially those using `async/await`.

-----

## ✨ Features

  - **Automatic Generation**: Creates methods like `print_field()` and `print()` for every decorated struct.
  - **Customizable Transporters**: Define your own message transport logic using the `#[transporter]` attribute.
  - **Integrated Log Levels**: Built-in support for the following fluid log levels:
      - `.err()`
      - `.info()`
      - `.success()`
      - `.warning()`
      - `.critical()`
      - `.panic()`
      - `.rust()` (useful for standard debug logs)
  - **Personalized Messages**: Add specific log messages to each call with the `.message()` method.
  - **`async/await` Compatible**: Fully integrated with the `tokio` ecosystem.

-----

## ⚙️ Installation

Add `nekoprint` to your `Cargo.toml`:

```bash
cargo add nekoprint
```

> Make sure the `nekoprint` macro is in scope.

-----

## 🚀 Usage

### Defining Your Struct

Decorate your struct with `#[derive(NekoPrint)]` and use the `#[transporter]` attribute to define how the log message will be processed. The transporter is an `async` function that takes a `message` variable (a `CString`).

```rust
use nekoprint::NekoPrint;

#[derive(Debug, NekoPrint, Default, Clone)]
#[transporter(async fn procedure() {
    println!("{message}");
    let m = message.to_string();
    assert!(
        m.contains("message")
    );
})]
pub struct User {
    id: i32,
    name: String,
    friend: Friend,
}

#[derive(Debug, NekoPrint, Default, Clone)]
#[transporter(async fn procedure() {
    println!("{message}");
    let m = message.to_string();
    assert!(
        m.contains("message")
    );
})]
pub struct Friend {
    id: i32,
    name: String,
}
```

### Full Usage Example

The `NekoPrint` macro generates methods like `print_id()`, `print_name()`, and `print()` for the `User` and `Friend` structs. You can then chain the `.message()` and `.level()` methods to customize the output.

```rust
#[tokio::test]
async fn test_print_user() {
    let user = User {
        id: 1,
        name: "name".into(),
        friend: Friend {
            id: 1,
            name: "name".into(),
        },
    };

    // Print the `id` field with different log levels
    user.print_id().message("custom message for id").err().await;
    user.print_id().message("info id message").info().await;
    user.print_id().message("success id message").success().await;
    user.print_id().message("warning id message").warning().await;
    user.print_id().message("critical id message").critical().await;
    user.print_id().message("panic id message").panic().await;
    user.print_id().message("rust debug id message").rust().await;

    // Print the `name` field with different log levels
    user.print_name().message("custom message for name").err().await;
    user.print_name().message("info name message").info().await;
    user.print_name().message("success name message").success().await;
    user.print_name().message("warning name message").warning().await;
    user.print_name().message("critical name message").critical().await;
    user.print_name().message("panic name message").panic().await;
    user.print_name().message("rust debug name message").rust().await;

    // Print the entire struct with different log levels
    user.print().message("custom message for all").err().await;
    user.print().message("custom message for all").info().await;
    user.print().message("custom message for all").success().await;
    user.print().message("custom message for all").warning().await;
    user.print().message("custom message for all").panic().await;
    user.print().message("custom message for all").critical().await;
    user.print().message("custom message for all").rust().await;
}
```

---

<h2 align="center">
  <strong>❤️ Donate</strong>
</h2>

<p align="center">
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Monero%20QR-FF6600?style=flat&logo=monero&logoColor=white" alt="Monero QR"/>
  </a>
  <a href="https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/BTC%20QR-EAB300?style=flat&logo=bitcoin&logoColor=white" alt="BTC QR"/>
  </a>
  <a href="https://revolut.me/pas2rust" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Revolut%20QR-Blue?style=flat&logo=revolut&logoColor=white" alt="Revolut QR"/>
  </a>
  <a href="https://wise.com/pay/me/pedroaugustos99" style="text-decoration:none; color:inherit;">
    <img src="https://img.shields.io/badge/Wise%20QR-1CA0F2?style=flat&logo=wise&logoColor=white" alt="Wise QR"/>
  </a>
</p>


---