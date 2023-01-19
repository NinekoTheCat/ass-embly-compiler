# ASS-embly spec

## : operator
defines a label with a name
`:<NAME>`
example:
```
:loop
```

labels can be jumped to like so
example:
```
:loop
JMP :loop
```
this program never ends

when jumping to a label the next instruction after the label gets executed

```
:loop
SUB $A $B $D
JMP :loop
```

will keep subtracting register A from B forever
## JHT
jumps to the label if the first argument is higher than the second argument
```
:loop
JHT ?a ?b :loop
```
## SUB operator

subtracts the first register from the second register outputting it into the third register
` SUB <register1> <register2> <output register>`

## JMP operator

Jumps to the label
` JMP :<label name>`
example:
```
:loop
JMP :loop
```

## DEF operator

defines a constant or ram address alias

example:
```
DEF $pi 3
DEF ?weed 420
```

## ? operator
describes a ram address to use with an optional value

example:
```
DEF ?cute
DEF ?cute 4
```

## MUL operator

multiplies the first and second variable outputting the result into the ram address
example:
```
MUL ?A ?B ?C
```

## ADD operator

adds the first and second variable and outputs the result into the ram address

```
ADD $A $B ?C
```

## CPY operator
Copies the A variable to the B register

```
CPY $A $B
```

## JNE operator
jumps if the values are not equal to each other
```
:loop
JNE $A $B :loop
```
## JE operator
jumps if the values are equal to each other
```
:loop
JE $A $B :loop
```
## DIV operator
divides the first and second variable outputting the result into the ram address
example:
```
DIV ?A ?B ?C
```
## '#' Operator
 the hashtag operator is a comment and will be ignored

## EXIT operator
input one register as the exit code and exits the program
```
EXIT ?arg1
```
## YEET operator
Writes data to the SECS ( Serial External Communication System ).
the first argument is the variable represeting the device ID
the second is the variable that holds the instruction
and the third argument is the address that points to the data
```
YEET ?arg1 ?arg2 ?arg3
```
### Roadmap
- '#' Operator Done
- Add operator done
- Sub operator done
- Def operator done
- ? operator done
- Exit operator done
- JE operator Done
- JNE operator Done
- CPY operator done
- MUL operator done
- JMP operator done
- : operator DONE
- DIV operator DONE