    .option norelax
    .section .text
    .global sync
sync:
    fence
    fence.i
    ret

    .global sync_data
sync_data:
    fence
    ret

sync_instruction:
    fence.i
    ret