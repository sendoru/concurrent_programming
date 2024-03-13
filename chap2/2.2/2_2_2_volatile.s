	.file	"2_2_2_volatile.c"
	.text
	.p2align 4
	.globl	wait_while_0
	.type	wait_while_0, @function
wait_while_0:
.LFB0:
	.cfi_startproc
	endbr64
	.p2align 4,,10
	.p2align 3
.L2:
	movl	(%rdi), %eax
	testl	%eax, %eax
	je	.L2
	ret
	.cfi_endproc
.LFE0:
	.size	wait_while_0, .-wait_while_0
	.ident	"GCC: (Ubuntu 9.4.0-1ubuntu1~20.04.2) 9.4.0"
	.section	.note.GNU-stack,"",@progbits
	.section	.note.gnu.property,"a"
	.align 8
	.long	 1f - 0f
	.long	 4f - 1f
	.long	 5
0:
	.string	 "GNU"
1:
	.align 8
	.long	 0xc0000002
	.long	 3f - 2f
2:
	.long	 0x3
3:
	.align 8
4:
