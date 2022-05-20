ldr r0, =0xE000ED08 // 向量表地址
ldr r0, [r0]
ldr r0, [r0]
msr msp, r0
mov r0, #0		
msr control, r0
cpsie i //使能全局中断
cpsie f //使能全局异常
dsb //数据同步
isb //指令同步
svc 0  //调用SVCall异常服务，在SVCall里恢复第一个任务
nop