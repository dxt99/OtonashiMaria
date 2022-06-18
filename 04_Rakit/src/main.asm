bits 64
default rel

segment .bss
buf: resb 4096

segment .data
    msg1 db "Input number to check: ", 0
    msg2 db "Number is prime", 0xd, 0xa, 0
    msg3 db "Number is not a prime", 0xd, 0xa, 0
    scanint db "%d"

segment .text
global main
extern ExitProcess

extern printf
extern scanf


isPrime:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 32
    
    cmp     rcx, 1
    je      .no

    mov     rbx, 2 ;iterator

.loop:
    cmp     rbx, rcx ;end loop
    je      .yes

    xor     rdx, rdx ;clear rdx
    mov     rax, rcx ;dividend
    div     rbx      ;divide

    cmp     rdx, 0   ;remainder is 0
    je      .no

    add     rbx, 1
    jmp     .loop

.yes:
    mov     rax, 1
    jmp     .exit

.no:
    mov     rax, 0

.exit:
    leave
    ret

main:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 32

    lea     rcx, [msg1]
    call    printf

    mov     rcx, scanint
    lea     rdx, [buf+8]
    call    scanf

    mov     rcx, [buf+8]
    call    isPrime

    xor     rcx, rcx
    and     rax, 1
    jz      .notprime

    lea     rcx, [msg2]
    jmp     .print

.notprime:
    lea     rcx, [msg3]
.print:
    call    printf
    xor     rax, rax
    call    ExitProcess