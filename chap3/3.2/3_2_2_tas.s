	.file	"3_2_2_tas.c"
	.text
	.p2align 4
	.globl	test_and_set
	.type	test_and_set, @function
test_and_set:
.LFB0:
	.cfi_startproc
	endbr64
	movl	$1, %eax
	xchgb	(%rdi), %al
	testb	%al, %al
	setne	%al
	ret
	.cfi_endproc
.LFE0:
	.size	test_and_set, .-test_and_set
	.p2align 4
	.globl	tas_release
	.type	tas_release, @function
tas_release:
.LFB1:
	.cfi_startproc
	endbr64
	movb	$0, (%rdi)
	ret
	.cfi_endproc
.LFE1:
	.size	tas_release, .-tas_release
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
