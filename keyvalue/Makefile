#
# Copyright 2016, NICTA
#
# This software may be distributed and modified according to the terms of
# the BSD 2-Clause license. Note that NO WARRANTY is provided.
# See "LICENSE_BSD2.txt" for details.
#
# @TAG(NICTA_BSD)
#
TARGETS := $(notdir ${SOURCE_DIR}).cdl
ADL := keyvalue.camkes

main_HFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/main/include/*.h)) \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/include/*.h))
main_RUST := main
main_CFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/main/src/*.c)) \
		$(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/main/crt/*.c))

store_HFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/store/include/*.h)) \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/include/*.h))
store_CFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/store/src/*.c)) \
		$(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/store/crt/*.c))

secondary_HFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/secondary/include/*.h)) \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/include/*.h))
secondary_CFILES := \
    $(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/secondary/src/*.c)) \
		$(patsubst ${SOURCE_DIR}/%,%,$(wildcard ${SOURCE_DIR}/components/secondary/crt/*.c))

store_LIBS := btreemap

include ${PWD}/tools/camkes/camkes.mk
