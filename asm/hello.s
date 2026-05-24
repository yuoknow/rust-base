.global _start

.text

_start:
    leaq    msg(%rip), %rsi # Кладем адрес строки в %rsi
    movq    $1, %rdi        # Кладем 1 в %rdi (stdout)
    movq    $1, %rax        # Кладем 1 в %rax (код системного вызова sys_write)
    movq    $14, %rdx       # Кладем 14 в %rdx (длина строки)
    syscall

    movq    $60, %rax       # Кладем 60 (код системного вызова exit) в регистр для вызова системных функций
    movq    $0, %rdi        # Кладем 0 в регистр для отображения кода выхода
    syscall

.section .rodata

msg:
    .string "Hello, World!\n"
