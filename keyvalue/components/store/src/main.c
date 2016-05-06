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
#include <utils/zf_log.h>

#define MAX_SIZE (sizeof(Buf)/sizeof(uint32_t))
#define MAX_COMPONENT_ID 2
#define MIN_COMPONENT_ID 1
#define NUM_COMPONENTS 2
CBTreeMap *btreemap;

volatile void *buffers[NUM_COMPONENTS];
//Initialise the btreemap
void pre_init(void) {
    buffers[0] = main_btreemap_buffer;
    buffers[1] = secondary_btreemap_buffer;
    btreemap = btreemap_new();
}

// The following functions wrap the btreemap.h functions and exposes them over
// a camkes rpc connection.
bool camkes_btreemap_contains_key(uint32_t key) {
    return btreemap_contains_key(btreemap, key);
}

OptionValue_culong camkes_btreemap_insert(uint32_t key, uint32_t value) {
    OptionValue_culong res;
    btreemap_insert(btreemap, key, value, &res);
    return res;
}

// Get sender id is used to find which component sent the message so that the response
// can be put into the correct buffer.
OptionVec_culong camkes_btreemap_keys() {
    OptionVec_culong res;
    int id = camkes_btreemap_get_sender_id();
    if (id < MIN_COMPONENT_ID || id > MAX_COMPONENT_ID) {
        ZF_LOGE("Invalid sender id");
        return res;
    }
    res.elements = (uint32_t*)buffers[id-1];
    res.max_size = MAX_SIZE;
    btreemap_keys(btreemap, &res);
    return res;

}

OptionVec_culong camkes_btreemap_values() {
    OptionVec_culong res;
    int id = camkes_btreemap_get_sender_id();
    if (id < MIN_COMPONENT_ID || id > MAX_COMPONENT_ID) {
        ZF_LOGE("Invalid sender id");
        return res;
    }
    res.elements = (uint32_t*)buffers[id-1];
    res.max_size = MAX_SIZE;
    btreemap_values(btreemap, &res);
    return res;
}

OptionValue_culong camkes_btreemap_remove(uint32_t key) {
    OptionValue_culong res;
    btreemap_remove(btreemap, key, &res);
    return res;
}

uintptr_t camkes_btreemap_len() {
    return btreemap_len(btreemap);
}

bool camkes_btreemap_is_empty() {
    return btreemap_is_empty(btreemap);
}
