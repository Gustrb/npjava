section .text
global _main 

_main:
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, message_len
    syscall

    mov rax, 60
    mov rdi, 0
    syscall

section .data
message:
    db "Hello, World!", 10
message_len:
    equ $ - message
