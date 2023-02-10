#ifndef __BINDINGS_HOST_H
#define __BINDINGS_HOST_H
#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
  int32_t *ptr;
  size_t len;
} host_list_s32_t;

typedef struct {
  host_list_s32_t a;
  int32_t b;
  uint64_t ptr;
  uint64_t c;
  uint64_t funcptr;
} host_s1_t;

typedef uint8_t host_e1_t;

#define HOST_E1_A 0
#define HOST_E1_B 1
#define HOST_E1_C 2

// Imported Functions from `host`
int32_t host_testfunc(host_s1_t *s, uint64_t a, host_e1_t t);

// Helper Functions

void host_list_s32_free(host_list_s32_t *ptr);
void host_s1_free(host_s1_t *ptr);

#ifdef __cplusplus
}
#endif
#endif
