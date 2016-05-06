/*
 * Copyright 2016, NICTA
 *
 * This software may be distributed and modified according to the terms of
 * the BSD 2-Clause license. Note that NO WARRANTY is provided.
 * See "LICENSE_BSD2.txt" for details.
 *
 * @TAG(NICTA_BSD)
 */

#include <camkes.h>
#include <stdio.h>
#include <btreemap.h>
#include <stdint.h>
#include <assert.h>

//This component adds a bunch of keys and values and then emits a barrier event and finishes

int run(void) {
    camkes_btreemap_insert(1, 2);
    camkes_btreemap_insert(2, 3);
    camkes_btreemap_insert(3, 4);
    camkes_btreemap_insert(4, 5);
    camkes_btreemap_insert(5, 6);
    OptionVec_culong res = camkes_btreemap_keys();
    printf("vec length: %d\n", res.len);
    uint32_t *a = (uint32_t *) btreemap_buffer;
    for (int i = 0; i < res.len; i++) {
        printf("key: %d\n", a[i]);
    }
    printf("Secondary done\n");
    barrier_event_emit();

    return 0;
}
