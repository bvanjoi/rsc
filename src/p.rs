#[macro_export]
macro_rules! head {
    ($offset: expr) => {
        format!(
            r#".globl main
main:
    push %rbp
    mov %rsp, %rbp
    sub ${}, %rsp
        "#,
            $offset
        )
    };
}

#[macro_export]
macro_rules! tail {
    () => {
        format!(
            r#"
    .L.return:
    mov %rbp, %rsp
    pop %rbp
    ret"#
        )
    };
}

#[macro_export]
/// push `%rax` to stack to project
macro_rules! push {
    () => {
        format!("push %rax")
    };
}

#[macro_export]
/// pop the value of stack to `$arg`.
macro_rules! pop {
    ($arg:expr) => {{
        format!("pop {}", $arg)
    }};
}
