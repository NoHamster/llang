S:= test.asm

obj= $(S:.asm=.o)
%.o: %.asm
	nasm -f elf64 $<

test: $(obj)
	ld $^ -o $@

clean:
	rm -rf $(obj) test
