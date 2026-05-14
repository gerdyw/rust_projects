.ORIG x3000

LEA R0, PROMPT
PUTS

LEA R1, BUFFER      ; R1 points to current buffer position

READ_LOOP
    GETC            ; read char into R0

    LD R2, NEG_NEWLINE
    ADD R2, R0, R2  ; R2 = char - newline
    BRz DONE_INPUT  ; if char == '\n', stop reading

    STR R0, R1, #0  ; store char at buffer pointer
    ADD R1, R1, #1  ; advance buffer pointer
    BRnzp READ_LOOP

DONE_INPUT
    AND R0, R0, #0
    STR R0, R1, #0  ; null-terminate string

    LEA R0, RESPONSE
    PUTS

    LEA R0, BUFFER
    PUTS

    LD R0, NEWLINE
    OUT

    HALT

PROMPT      .STRINGZ "Type a line: "
RESPONSE    .STRINGZ "You typed: "
NEWLINE     .FILL x000A
NEG_NEWLINE .FILL xFFF6     ; -10, because newline is ASCII 10

BUFFER      .BLKW 80

.END