#[macro_export]
macro_rules! head {
    () => {
        println!(
            r#".globl main
main:
        "#
        )
    };
}

#[macro_export]
macro_rules! tail {
    () => {
        println!(
            r#"
ret
"#
        )
    };
}

#[macro_export]
macro_rules! push {
    () => {
        println!("push %rax")
    };
}

#[macro_export]
macro_rules! pop {
    ($arg:expr) => {{
        println!("pop {}", $arg)
    }};
}
