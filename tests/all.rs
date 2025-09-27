use nekoprint::NekoPrint;

#[derive(Debug, NekoPrint)]
#[transporter(async fn procedure() {
   println!("{message}");
   let m = message.to_string();
    assert!(
        m.contains("custom message") || m.contains("#")
    );
})]
pub struct User {
    id: i32,
    name: String,
}

#[tokio::test]
async fn all_print_methods_do_not_panic() {
    let user = User {
        id: 1,
        name: "name".into(),
    };

    user.print("custom message").await;
    user.print_success("#").await;
    user.print_warning("#").await;
    user.print_critical("#").await;
    user.print_err("#").await;
    user.print_info("#").await;
    user.print_panic("#").await;
    user.print_rust("#").await;

    user.print_name("custom message").await;
    user.print_success_name("#").await;
    user.print_warning_name("#").await;
    user.print_critical_name("#").await;
    user.print_err_name("#").await;
    user.print_info_name("#").await;
    user.print_panic_name("#").await;
    user.print_rust_name("#").await;
}
