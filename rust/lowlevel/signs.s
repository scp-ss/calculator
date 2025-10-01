.section .data
signs:     .asciz "+ - * / \n"
len:       .quad 8              # Length of the string

.section .text
.global _start

_start:
    mov     $1, %rax            # syscall: write
    mov     $1, %rdi            # file descriptor: stdout
    lea     signs(%rip), %rsi   # pointer to string
    mov     len(%rip), %rdx     # length of string
    syscall

    mov     $60, %rax           # syscall: exit
    xor     %rdi, %rdi          # exit code 0
    syscall
