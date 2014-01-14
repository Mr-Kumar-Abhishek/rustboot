//===-- aeabi_ldivmod.S - EABI ldivmod implementation ---------------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is dual licensed under the MIT and the University of Illinois Open
// Source Licenses. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//

// struct { int64_t quot, int64_t rem}
//        __aeabi_ldivmod(int64_t numerator, int64_t denominator) {
//   int64_t rem, quot;
//   quot = __divmoddi4(numerator, denominator, &rem);
//   return {quot, rem};
// }

        .syntax unified
        .align 2
.globl __aeabi_ldivmod
__aeabi_ldivmod:
        push    {r11, lr}
        sub     sp, sp, #16
        add     r12, sp, #8
        str     r12, [sp]
        bl      __divmoddi4
        ldr     r2, [sp, #8]
        ldr     r3, [sp, #12]
        add     sp, sp, #16
        pop     {r11, pc}

        .align 2
.globl __aeabi_uldivmod
__aeabi_uldivmod:
        push    {r11, lr}
        sub     sp, sp, #16
        add     r12, sp, #8
        str     r12, [sp]
        bl      __udivmoddi4
        ldr     r2, [sp, #8]
        ldr     r3, [sp, #12]
        add     sp, sp, #16
        pop     {r11, pc}
