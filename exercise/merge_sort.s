	.file	"merge_sort.c"
	.text
	.p2align 4
	.globl	insertion_sort
	.type	insertion_sort, @function
insertion_sort:
.LFB40:
	.cfi_startproc
	endbr64
	leal	1(%rsi), %eax
	cmpl	%edx, %eax
	jg	.L12
	cltq
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	%esi, %r11d
	movl	%edx, %ebx
	leaq	(%rdi,%rax,4), %rdx
	.p2align 4,,10
	.p2align 3
.L7:
	movl	(%rdx), %r10d
	movq	%rdx, %r9
	movl	%r11d, %ecx
	cmpl	%r11d, %esi
	jg	.L4
	movslq	%r11d, %rax
	leaq	4(%rdi,%rax,4), %rax
	jmp	.L5
	.p2align 4,,10
	.p2align 3
.L6:
	subl	$1, %ecx
	movl	%r8d, (%rax)
	leaq	-4(%r9), %rax
	cmpl	%ecx, %esi
	jg	.L16
.L5:
	movl	-4(%rax), %r8d
	movq	%rax, %r9
	cmpl	%r10d, %r8d
	jg	.L6
.L4:
	addl	$1, %r11d
	movl	%r10d, (%r9)
	addq	$4, %rdx
	cmpl	%r11d, %ebx
	jne	.L7
.L17:
	popq	%rbx
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L16:
	.cfi_restore_state
	movslq	%ecx, %rcx
	addl	$1, %r11d
	addq	$4, %rdx
	leaq	4(%rdi,%rcx,4), %r9
	movl	%r10d, (%r9)
	cmpl	%r11d, %ebx
	jne	.L7
	jmp	.L17
.L12:
	.cfi_def_cfa_offset 8
	.cfi_restore 3
	ret
	.cfi_endproc
.LFE40:
	.size	insertion_sort, .-insertion_sort
	.p2align 4
	.globl	merge
	.type	merge, @function
merge:
.LFB41:
	.cfi_startproc
	endbr64
	movl	%edx, %r8d
	pushq	%r15
	.cfi_def_cfa_offset 16
	.cfi_offset 15, -16
	subl	%edx, %ecx
	pushq	%r14
	.cfi_def_cfa_offset 24
	.cfi_offset 14, -24
	subl	%esi, %r8d
	pushq	%r13
	.cfi_def_cfa_offset 32
	.cfi_offset 13, -32
	leal	1(%r8), %r9d
	movq	%rdi, %r13
	pushq	%r12
	.cfi_def_cfa_offset 40
	.cfi_offset 12, -40
	movslq	%r9d, %rdi
	movl	%ecx, %r12d
	pushq	%rbp
	.cfi_def_cfa_offset 48
	.cfi_offset 6, -48
	salq	$2, %rdi
	movl	%esi, %ebp
	pushq	%rbx
	.cfi_def_cfa_offset 56
	.cfi_offset 3, -56
	movl	%edx, %ebx
	subq	$24, %rsp
	.cfi_def_cfa_offset 80
	movl	%r8d, 12(%rsp)
	movl	%r9d, 8(%rsp)
	call	malloc@PLT
	movslq	%r12d, %rdi
	salq	$2, %rdi
	movq	%rax, %r15
	call	malloc@PLT
	movl	8(%rsp), %r9d
	movl	12(%rsp), %r8d
	movq	%rax, %r14
	testl	%r9d, %r9d
	jle	.L45
	movl	%r8d, %eax
	movq	%r15, %rdi
	movl	%r9d, 12(%rsp)
	leaq	4(,%rax,4), %rdx
	movslq	%ebp, %rax
	movl	%r8d, 8(%rsp)
	leaq	0(%r13,%rax,4), %rsi
	call	memcpy@PLT
	testl	%r12d, %r12d
	movl	8(%rsp), %r8d
	movl	12(%rsp), %r9d
	jle	.L21
	addl	$1, %ebx
	leal	-1(%r12), %eax
	movq	%r14, %rdi
	movl	%r9d, 12(%rsp)
	movslq	%ebx, %rbx
	leaq	4(,%rax,4), %rdx
	movl	%r8d, 8(%rsp)
	leaq	0(%r13,%rbx,4), %rsi
	xorl	%ebx, %ebx
	call	memcpy@PLT
	leal	1(%rbp), %eax
	movl	8(%rsp), %r8d
	xorl	%edx, %edx
	movl	12(%rsp), %r9d
	cltq
	xorl	%r10d, %r10d
	xorl	%edi, %edi
	.p2align 4,,10
	.p2align 3
.L29:
	movl	(%r15,%r10,4), %ecx
	movl	(%r14,%rdi,4), %esi
	cmpl	%esi, %ecx
	jg	.L25
.L46:
	movl	%ecx, -4(%r13,%rax,4)
	movl	%eax, %ebp
	leal	1(%rdx), %ecx
	addq	$1, %rax
	cmpl	%edx, %r8d
	jle	.L23
	cmpl	%ebx, %r12d
	jle	.L23
	movslq	%ecx, %r10
	movl	(%r14,%rdi,4), %esi
	movl	(%r15,%r10,4), %ecx
	movq	%r10, %rdx
	cmpl	%esi, %ecx
	jle	.L46
.L25:
	movl	%esi, -4(%r13,%rax,4)
	movl	%eax, %ebp
	addl	$1, %ebx
	addq	$1, %rax
	cmpl	%edx, %r9d
	jle	.L33
	cmpl	%r12d, %ebx
	jge	.L33
	movslq	%ebx, %rdi
	jmp	.L29
	.p2align 4,,10
	.p2align 3
.L21:
	xorl	%ebx, %ebx
	xorl	%ecx, %ecx
	.p2align 4,,10
	.p2align 3
.L23:
	cmpl	%ecx, %r9d
	movl	%r9d, 8(%rsp)
	jle	.L30
	movslq	%ebp, %rax
	subl	%ecx, %r8d
	movl	%ecx, 12(%rsp)
	leaq	0(%r13,%rax,4), %rdi
	movslq	%ecx, %rax
	leaq	4(,%r8,4), %rdx
	leaq	(%r15,%rax,4), %rsi
	call	memcpy@PLT
	movl	8(%rsp), %r9d
	movl	12(%rsp), %ecx
	addl	%r9d, %ebp
	subl	%ecx, %ebp
.L30:
	leal	-1(%r12), %ecx
	cmpl	%ebx, %r12d
	jle	.L31
.L24:
	subl	%ebx, %ecx
	movslq	%ebp, %rbp
	movslq	%ebx, %rbx
	leaq	0(%r13,%rbp,4), %rdi
	leaq	4(,%rcx,4), %rdx
	leaq	(%r14,%rbx,4), %rsi
	call	memcpy@PLT
.L31:
	movq	%r15, %rdi
	call	free@PLT
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 56
	movq	%r14, %rdi
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%rbp
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	jmp	free@PLT
	.p2align 4,,10
	.p2align 3
.L45:
	.cfi_restore_state
	testl	%r12d, %r12d
	jle	.L21
	addl	$1, %ebx
	leal	-1(%r12), %eax
	movq	%r14, %rdi
	movslq	%ebx, %rbx
	leaq	4(,%rax,4), %rdx
	movl	%eax, 8(%rsp)
	leaq	0(%r13,%rbx,4), %rsi
	xorl	%ebx, %ebx
	call	memcpy@PLT
	movl	8(%rsp), %ecx
	jmp	.L24
	.p2align 4,,10
	.p2align 3
.L33:
	movl	%edx, %ecx
	jmp	.L23
	.cfi_endproc
.LFE41:
	.size	merge, .-merge
	.p2align 4
	.globl	merge_sort_single_thread
	.type	merge_sort_single_thread, @function
merge_sort_single_thread:
.LFB42:
	.cfi_startproc
	endbr64
	pushq	%r14
	.cfi_def_cfa_offset 16
	.cfi_offset 14, -16
	movl	%edx, %eax
	pushq	%r13
	.cfi_def_cfa_offset 24
	.cfi_offset 13, -24
	subl	%esi, %eax
	movq	%rdi, %r13
	pushq	%r12
	.cfi_def_cfa_offset 32
	.cfi_offset 12, -32
	movl	%edx, %r12d
	pushq	%rbp
	.cfi_def_cfa_offset 40
	.cfi_offset 6, -40
	movl	%esi, %ebp
	subq	$8, %rsp
	.cfi_def_cfa_offset 48
	cmpl	$31, %eax
	jle	.L60
	cmpl	%esi, %edx
	jg	.L61
.L47:
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 40
	popq	%rbp
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r13
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L61:
	.cfi_restore_state
	sarl	%eax
	leal	(%rax,%rsi), %r14d
	movl	%r14d, %edx
	call	merge_sort_single_thread
	leal	1(%r14), %esi
	movl	%r12d, %edx
	movq	%r13, %rdi
	call	merge_sort_single_thread
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 40
	movl	%r12d, %ecx
	movl	%r14d, %edx
	movl	%ebp, %esi
	movq	%r13, %rdi
	popq	%rbp
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r13
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmp	merge
	.p2align 4,,10
	.p2align 3
.L60:
	.cfi_restore_state
	leal	1(%rsi), %eax
	cmpl	%eax, %edx
	jl	.L47
	cltq
	movl	%esi, %edx
	leaq	(%rdi,%rax,4), %rdi
	.p2align 4,,10
	.p2align 3
.L55:
	movl	(%rdi), %r8d
	movq	%rdi, %rsi
	movl	%edx, %eax
	cmpl	%edx, %ebp
	jg	.L52
	movslq	%edx, %rcx
	leaq	4(%r13,%rcx,4), %rcx
	jmp	.L53
	.p2align 4,,10
	.p2align 3
.L54:
	subl	$1, %eax
	movl	%r9d, (%rcx)
	leaq	-4(%rsi), %rcx
	cmpl	%eax, %ebp
	jg	.L62
.L53:
	movl	-4(%rcx), %r9d
	movq	%rcx, %rsi
	cmpl	%r9d, %r8d
	jl	.L54
.L52:
	addl	$1, %edx
	movl	%r8d, (%rsi)
	addq	$4, %rdi
	cmpl	%edx, %r12d
	jne	.L55
.L63:
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 40
	popq	%rbp
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r13
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L62:
	.cfi_restore_state
	cltq
	addl	$1, %edx
	addq	$4, %rdi
	leaq	4(%r13,%rax,4), %rsi
	movl	%r8d, (%rsi)
	cmpl	%edx, %r12d
	jne	.L55
	jmp	.L63
	.cfi_endproc
.LFE42:
	.size	merge_sort_single_thread, .-merge_sort_single_thread
	.section	.rodata.str1.1,"aMS",@progbits,1
.LC0:
	.string	"pthread_create"
.LC1:
	.string	"pthread_join"
	.text
	.p2align 4
	.globl	__merge_sort_multi_thread
	.type	__merge_sort_multi_thread, @function
__merge_sort_multi_thread:
.LFB43:
	.cfi_startproc
	endbr64
	pushq	%r15
	.cfi_def_cfa_offset 16
	.cfi_offset 15, -16
	pushq	%r14
	.cfi_def_cfa_offset 24
	.cfi_offset 14, -24
	pushq	%r13
	.cfi_def_cfa_offset 32
	.cfi_offset 13, -32
	pushq	%r12
	.cfi_def_cfa_offset 40
	.cfi_offset 12, -40
	pushq	%rbp
	.cfi_def_cfa_offset 48
	.cfi_offset 6, -48
	subq	$80, %rsp
	.cfi_def_cfa_offset 128
	movl	12(%rdi), %r12d
	movl	8(%rdi), %ebp
	movq	%fs:40, %rax
	movq	%rax, 72(%rsp)
	xorl	%eax, %eax
	movq	(%rdi), %r13
	movl	%r12d, %eax
	subl	%ebp, %eax
	cmpl	$31, %eax
	jle	.L84
	cmpl	%r12d, %ebp
	jl	.L85
.L67:
	movq	72(%rsp), %rax
	xorq	%fs:40, %rax
	jne	.L86
	addq	$80, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 48
	xorl	%eax, %eax
	popq	%rbp
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L85:
	.cfi_restore_state
	sarl	%eax
	movl	16(%rdi), %edx
	leal	(%rax,%rbp), %r14d
	leal	1(%r14), %r15d
	cmpl	$3, %edx
	jg	.L74
	addl	$1, %edx
	xorl	%esi, %esi
	leaq	16(%rsp), %rcx
	movq	%rsp, %rdi
	movl	%edx, 32(%rsp)
	movl	%edx, 64(%rsp)
	leaq	__merge_sort_multi_thread(%rip), %rdx
	movl	%ebp, 24(%rsp)
	movl	%r14d, 28(%rsp)
	movl	%r15d, 56(%rsp)
	movl	%r12d, 60(%rsp)
	movq	%r13, 16(%rsp)
	movq	%r13, 48(%rsp)
	call	pthread_create@PLT
	testl	%eax, %eax
	jne	.L76
	xorl	%esi, %esi
	leaq	48(%rsp), %rcx
	leaq	8(%rsp), %rdi
	leaq	__merge_sort_multi_thread(%rip), %rdx
	call	pthread_create@PLT
	testl	%eax, %eax
	jne	.L76
	movq	(%rsp), %rdi
	xorl	%esi, %esi
	call	pthread_join@PLT
	testl	%eax, %eax
	jne	.L78
	movq	8(%rsp), %rdi
	xorl	%esi, %esi
	call	pthread_join@PLT
	testl	%eax, %eax
	je	.L79
.L78:
	leaq	.LC1(%rip), %rdi
	call	perror@PLT
	orl	$-1, %edi
	call	exit@PLT
	.p2align 4,,10
	.p2align 3
.L84:
	leal	1(%rbp), %eax
	cmpl	%eax, %r12d
	jl	.L67
	cltq
	movl	%ebp, %edx
	leaq	0(%r13,%rax,4), %rdi
	.p2align 4,,10
	.p2align 3
.L72:
	movl	(%rdi), %r8d
	movq	%rdi, %rsi
	movl	%edx, %eax
	cmpl	%edx, %ebp
	jg	.L69
	movslq	%edx, %rcx
	leaq	4(%r13,%rcx,4), %rcx
	jmp	.L70
	.p2align 4,,10
	.p2align 3
.L71:
	subl	$1, %eax
	movl	%r9d, (%rcx)
	leaq	-4(%rsi), %rcx
	cmpl	%eax, %ebp
	jg	.L87
.L70:
	movl	-4(%rcx), %r9d
	movq	%rcx, %rsi
	cmpl	%r9d, %r8d
	jl	.L71
.L69:
	addl	$1, %edx
	movl	%r8d, (%rsi)
	addq	$4, %rdi
	cmpl	%edx, %r12d
	jne	.L72
	jmp	.L67
	.p2align 4,,10
	.p2align 3
.L74:
	movl	%r14d, %edx
	movl	%ebp, %esi
	movq	%r13, %rdi
	call	merge_sort_single_thread
	movl	%r12d, %edx
	movl	%r15d, %esi
	movq	%r13, %rdi
	call	merge_sort_single_thread
.L79:
	movl	%r12d, %ecx
	movl	%r14d, %edx
	movl	%ebp, %esi
	movq	%r13, %rdi
	call	merge
	jmp	.L67
	.p2align 4,,10
	.p2align 3
.L87:
	cltq
	addl	$1, %edx
	addq	$4, %rdi
	leaq	4(%r13,%rax,4), %rsi
	movl	%r8d, (%rsi)
	cmpl	%edx, %r12d
	jne	.L72
	jmp	.L67
.L86:
	call	__stack_chk_fail@PLT
.L76:
	leaq	.LC0(%rip), %rdi
	call	perror@PLT
	orl	$-1, %edi
	call	exit@PLT
	.cfi_endproc
.LFE43:
	.size	__merge_sort_multi_thread, .-__merge_sort_multi_thread
	.p2align 4
	.globl	merge_sort_multi_thread
	.type	merge_sort_multi_thread, @function
merge_sort_multi_thread:
.LFB44:
	.cfi_startproc
	endbr64
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%fs:40, %rax
	movq	%rax, 24(%rsp)
	xorl	%eax, %eax
	movq	%rdi, (%rsp)
	movq	%rsp, %rdi
	movl	%esi, 8(%rsp)
	movl	%edx, 12(%rsp)
	movl	$0, 16(%rsp)
	call	__merge_sort_multi_thread
	movq	24(%rsp), %rax
	xorq	%fs:40, %rax
	jne	.L91
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
.L91:
	.cfi_restore_state
	call	__stack_chk_fail@PLT
	.cfi_endproc
.LFE44:
	.size	merge_sort_multi_thread, .-merge_sort_multi_thread
	.section	.rodata.str1.1
.LC2:
	.string	"Usage: %s <n>\n"
.LC3:
	.string	"%d "
	.section	.rodata.str1.8,"aMS",@progbits,1
	.align 8
.LC4:
	.string	"n = %d, skipped printing elements for too large n.\n"
	.align 8
.LC6:
	.string	"Time for multi-thread merge sort: %lf\n"
	.align 8
.LC7:
	.string	"Time for single-thread merge sort: %lf\n"
	.section	.rodata.str1.1
.LC8:
	.string	"Sorted array is "
.LC9:
	.string	"Array is not sorted"
.LC10:
	.string	"Array is sorted"
.LC11:
	.string	"Generated array is "
	.section	.text.startup,"ax",@progbits
	.p2align 4
	.globl	main
	.type	main, @function
main:
.LFB45:
	.cfi_startproc
	endbr64
	pushq	%r15
	.cfi_def_cfa_offset 16
	.cfi_offset 15, -16
	pushq	%r14
	.cfi_def_cfa_offset 24
	.cfi_offset 14, -24
	pushq	%r13
	.cfi_def_cfa_offset 32
	.cfi_offset 13, -32
	pushq	%r12
	.cfi_def_cfa_offset 40
	.cfi_offset 12, -40
	pushq	%rbp
	.cfi_def_cfa_offset 48
	.cfi_offset 6, -48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	.cfi_offset 3, -56
	subq	$104, %rsp
	.cfi_def_cfa_offset 160
	movq	%fs:40, %rax
	movq	%rax, 88(%rsp)
	xorl	%eax, %eax
	cmpl	$1, %edi
	jle	.L117
	movq	8(%rsi), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtol@PLT
	movq	%rax, %r13
	cltq
	salq	$2, %rax
	movq	%rax, %rdi
	movq	%rax, 16(%rsp)
	call	malloc@PLT
	movq	%rax, %r14
	testl	%r13d, %r13d
	jle	.L95
	movq	%rax, %rbp
	leal	-1(%r13), %eax
	leal	0(%r13,%r13,2), %ebx
	movq	%r14, %r15
	movl	%eax, 12(%rsp)
	leaq	4(%r14,%rax,4), %r12
	movq	%rax, 24(%rsp)
	.p2align 4,,10
	.p2align 3
.L96:
	call	rand@PLT
	addq	$4, %r15
	cltd
	idivl	%ebx
	movl	%edx, -4(%r15)
	cmpq	%r12, %r15
	jne	.L96
	movq	16(%rsp), %rdi
	call	malloc@PLT
	movq	24(%rsp), %rcx
	movq	%r14, %rsi
	movq	%rax, %rdi
	movq	%rax, %r15
	leaq	4(,%rcx,4), %rdx
	call	memcpy@PLT
	cmpl	$100, %r13d
	jg	.L118
	leaq	.LC11(%rip), %rdi
	leaq	.LC3(%rip), %rbx
	call	puts@PLT
	.p2align 4,,10
	.p2align 3
.L98:
	movl	0(%rbp), %edx
	movq	%rbx, %rsi
	movl	$1, %edi
	xorl	%eax, %eax
	addq	$4, %rbp
	call	__printf_chk@PLT
	cmpq	%r12, %rbp
	jne	.L98
.L108:
	movl	$10, %edi
	call	putchar@PLT
	jmp	.L99
.L118:
	movl	%r13d, %edx
	leaq	.LC4(%rip), %rsi
	movl	$1, %edi
	xorl	%eax, %eax
	call	__printf_chk@PLT
.L99:
	leaq	32(%rsp), %r12
	xorl	%edi, %edi
	leaq	48(%rsp), %rbp
	movq	%r12, %rsi
	call	clock_gettime@PLT
	movl	12(%rsp), %ebx
	leaq	64(%rsp), %rdi
	movl	$0, 72(%rsp)
	movl	$0, 80(%rsp)
	movl	%ebx, 76(%rsp)
	movq	%r14, 64(%rsp)
	call	__merge_sort_multi_thread
	xorl	%edi, %edi
	movq	%rbp, %rsi
	call	clock_gettime@PLT
	movq	56(%rsp), %rax
	pxor	%xmm0, %xmm0
	subq	40(%rsp), %rax
	cvtsi2sdq	%rax, %xmm0
	pxor	%xmm1, %xmm1
	movq	48(%rsp), %rax
	subq	32(%rsp), %rax
	cvtsi2sdq	%rax, %xmm1
	movl	$1, %edi
	movl	$1, %eax
	divsd	.LC5(%rip), %xmm0
	leaq	.LC6(%rip), %rsi
	addsd	%xmm1, %xmm0
	call	__printf_chk@PLT
	xorl	%edi, %edi
	movq	%r12, %rsi
	call	clock_gettime@PLT
	xorl	%esi, %esi
	movl	%ebx, %edx
	movq	%r15, %rdi
	call	merge_sort_single_thread
	xorl	%edi, %edi
	movq	%rbp, %rsi
	call	clock_gettime@PLT
	movq	56(%rsp), %rax
	pxor	%xmm0, %xmm0
	subq	40(%rsp), %rax
	cvtsi2sdq	%rax, %xmm0
	pxor	%xmm1, %xmm1
	movq	48(%rsp), %rax
	subq	32(%rsp), %rax
	cvtsi2sdq	%rax, %xmm1
	movl	$1, %edi
	movl	$1, %eax
	divsd	.LC5(%rip), %xmm0
	leaq	.LC7(%rip), %rsi
	addsd	%xmm1, %xmm0
	call	__printf_chk@PLT
	cmpl	$100, %r13d
	jle	.L119
.L100:
	movl	12(%rsp), %eax
	testl	%eax, %eax
	jle	.L103
	leal	-2(%r13), %ecx
	movl	(%r14), %edx
	leaq	4(%r14), %rax
	leaq	8(%r14,%rcx,4), %rsi
	jmp	.L105
	.p2align 4,,10
	.p2align 3
.L104:
	addq	$4, %rax
	cmpq	%rsi, %rax
	je	.L103
.L105:
	movl	%edx, %ecx
	movl	(%rax), %edx
	cmpl	%ecx, %edx
	jge	.L104
	leaq	.LC9(%rip), %rdi
	call	puts@PLT
	xorl	%eax, %eax
.L92:
	movq	88(%rsp), %rcx
	xorq	%fs:40, %rcx
	jne	.L120
	addq	$104, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%rbp
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	ret
.L103:
	.cfi_restore_state
	leaq	.LC10(%rip), %rdi
	call	puts@PLT
	xorl	%eax, %eax
	jmp	.L92
.L119:
	leaq	.LC8(%rip), %rdi
	call	puts@PLT
	testl	%r13d, %r13d
	jle	.L101
	movl	12(%rsp), %eax
	movq	%r14, %rbx
	leaq	.LC3(%rip), %rbp
	leaq	4(%r14,%rax,4), %r12
	.p2align 4,,10
	.p2align 3
.L102:
	movl	(%rbx), %edx
	movq	%rbp, %rsi
	movl	$1, %edi
	xorl	%eax, %eax
	addq	$4, %rbx
	call	__printf_chk@PLT
	cmpq	%rbx, %r12
	jne	.L102
.L101:
	movl	$10, %edi
	call	putchar@PLT
	jmp	.L100
.L95:
	movq	16(%rsp), %rdi
	call	malloc@PLT
	leaq	.LC11(%rip), %rdi
	movq	%rax, %r15
	call	puts@PLT
	leal	-1(%r13), %eax
	movl	%eax, 12(%rsp)
	jmp	.L108
.L117:
	movq	(%rsi), %rdx
	movl	$1, %edi
	leaq	.LC2(%rip), %rsi
	call	__printf_chk@PLT
	orl	$-1, %eax
	jmp	.L92
.L120:
	call	__stack_chk_fail@PLT
	.cfi_endproc
.LFE45:
	.size	main, .-main
	.section	.rodata.cst8,"aM",@progbits,8
	.align 8
.LC5:
	.long	0
	.long	1104006501
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
