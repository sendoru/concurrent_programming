	.file	"3_2_1_cas_1.c"
	.text
	.p2align 4
	.globl	compare_and_swap
	.type	compare_and_swap, @function
compare_and_swap:
.LFB0:
	.cfi_startproc
	endbr64
	xorl	%eax, %eax		; %eax = 0
	cmpq	%rsi, (%rdi)	; %rsi == %rdi? (*p == val?)
	jne	.L1					; if diff jump .L1
	movq	%rdx, (%rdi)	; %rdi = %rdx (*p = newval)
	movl	$1, %eax
.L1:
	ret
	.cfi_endproc
.LFE0:
	.size	compare_and_swap, .-compare_and_swap
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
