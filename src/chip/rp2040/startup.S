ldr r3, = CURRENT_TASK_PTR /* Obtain location of pxCurrentTCB. */
ldr r1, [ r3 ]
ldr r0, [ r1 ]         /* The first item in pxCurrentTCB is the task top of stack. */
adds r0, # 32          /* Discard everything up to r0. */
msr psp, r0            /* This is now the new top of stack to use in the task. */
movs r0, # 2           /* Switch to the psp stack. */
msr CONTROL, r0
isb
pop {{ r0 - r5 }} /* Pop the registers that are saved automatically. */
mov lr, r5 /* lr is now in r5. */
pop {{ r3 }} /* The return address is now in r3. */
pop {{ r2 }} /* Pop and discard the XPSR. */
cpsie i /* The first task has its context and interrupts can be enabled. */
cpsie f
bx r3 /* Finally, jump to the user defined task code. */