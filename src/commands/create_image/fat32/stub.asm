; Assembly for the boot stub located after the BPB
; Assembled using NASM with "-f bin" 

[ORG 0x7C5A]
 
start:
    xor ax, ax
    mov ds, ax
    
    mov si, msg
    cld

    .print_loop:
        lodsb
        or al, al
        jz .hang

        mov ah, 0x0E
        mov bh, 0
        int 0x10
        jmp .print_loop
    
    .hang:
        cli
        hlt
        jmp .hang
 
msg db 'LanceOS only supports UEFI', 13, 10, 0