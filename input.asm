addi x1, x0, 0
addi x2, x0, 10
addi x3, x0, 2

loop:

bge x1, x2, end
bne x1, x3, false
addi x4, x0, 0
false:
addi x4, x0, 1
addi x1, x1, 1
j loop
end:
finish