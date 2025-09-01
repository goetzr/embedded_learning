    .section .vector_table.reset_vector, "a"
    .align 2
    .long Reset

    .section .vector_table.exceptions, "a"
    .align 2
    .dcb.l 5, DefaultHandler
    .dcb.l 4, 0
    .dc.l DefaultHandler
    .dcb.l 2, 0
    .dcb.l 2, DefaultHandler

    .section .vector_table.interrupts, "a"
    .align 2
    .dcb.l 240, DefaultHandler
