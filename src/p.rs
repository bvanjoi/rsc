#[macro_export]
macro_rules! head {
    () => {
        println!(
            r#".globl main
main:
    push %rbp
    mov %rsp, %rbp
    sub $208, %rsp
        "#
        )
    };
}

#[macro_export]
macro_rules! tail {
    () => {
        println!(
            r#"
    .L.return:
    mov %rbp, %rsp
    pop %rbp
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
