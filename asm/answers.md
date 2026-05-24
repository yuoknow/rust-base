# Bomb Answers

## Phase 1

Увидел что сравнение с какой-то строкой. Вызвал:

```bash
objdump -s -j .rodata -d bomb_hard --no-show-raw-insn
```

Увидел строку похожую на ответ.

**Ответ:** `Rust is blazingly fast and memory-efficient`

---

## Phase 2

Первое число 1:

```asm
cmpl   $0x1,-0x20(%rbp)
```

Потом увидел смещение бита в цикле:

```asm
shl    $1,%ecx
```

**Ответ:** `1 2 4 8 16 32`

---

## Phase 3

Первое число от 0 до 5:

```asm
40130e: sub    $0x5,%rax
401312: ja     4013ad <phase_3+0xdd>
```

Далее вычисляется адрес куда прыгнуть в зависимости от первого числа:

```asm
401318: mov    -0x18(%rbp),%rax
40131c: mov    0x402008(,%rax,8),%rax
401324: jmp    *%rax
```

Смотрим что там лежит по этим адресам:

```gdb
gdb ./bomb_hard
x/6a 0x402008
```

Если ноль, то: `0x402008: 0x401326` — переходим по адресу 401326:

```asm
cmpl   $0x1c4,-0x10(%rbp)
```

Сравнение с константой 452.

**Ответ:** `0 452`

---

## Phase 4

Число больше 1:

```asm
401470: cmpl   $0x1,-0x24(%rbp)
401474: jge    40147f <phase_4+0x3f>
```

Число меньше 12:

```asm
401489: cmpl   $0xc,-0xc(%rbp)
40148d: jle    401498 <phase_4+0x58>
```

Вызов функции фибоначчи:

```asm
40149b: call   4013d0 <fib>
```

Результат сравнивается с 55:

```asm
4014a0: cmp    $0x37,%eax
```

55 — это 10 число фибоначчи.

Также через `objdump` найдена строка `blazingpower`:

```bash
objdump -s -j .rodata -d bomb_hard --no-show-raw-insn
```

**Ответ:** `10 blazingpower`

---

## Secret Phase

Сравнение с константой 1337:

```asm
401559: cmpl   $0x539,-0x74(%rbp)
```

**Ответ:** `1337`
