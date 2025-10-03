section .text
global _main
_main:
        L0:
        ;; todo: implement get static
        L3:
        push qword 13
        lea r10, [rel data_section_elements + 0]
        push r10
        L5:
        call runtime$println
        L8:
        mov rax, 0x2000001
        mov rdi, 0
        syscall

runtime$println:

mov rsi, qword [rsp + 8]
        mov rdx, qword [rsp + 16]
        mov rax, 0x2000004
        mov rdi, 1
        syscall
        ret

section .data
global data_section_elements
data_section_elements:
        db "Hello, World!", 10