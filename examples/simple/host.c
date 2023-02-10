#include "host.h"


__attribute__((import_module("$root"), import_name("testfunc")))
int32_t __wasm_import_host_testfunc(int32_t, int32_t, int32_t, int64_t, int64_t, int64_t, int64_t, int32_t);

__attribute__((weak, export_name("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  if (new_size == 0) return (void*) align;
  void *ret = realloc(ptr, new_size);
  if (!ret) abort();
  return ret;
}

// Helper Functions

void host_list_s32_free(host_list_s32_t *ptr) {
  if (ptr->len > 0) {
    free(ptr->ptr);
  }
}

void host_s1_free(host_s1_t *ptr) {
  host_list_s32_free(&ptr->a);
}

// Component Adapters

int32_t host_testfunc(host_s1_t *s, uint64_t a, host_e1_t t) {
  int32_t ret = __wasm_import_host_testfunc((int32_t) ((*s).a).ptr, (int32_t) ((*s).a).len, (*s).b, (int64_t) ((*s).ptr), (int64_t) ((*s).c), (int64_t) ((*s).funcptr), (int64_t) (a), (int32_t) t);
  return ret;
}

extern void __component_type_object_force_link_host(void);
void __component_type_object_force_link_host_public_use_in_this_compilation_unit(void) {
  __component_type_object_force_link_host();
}
