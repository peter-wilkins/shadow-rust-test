#if !defined(_GNU_SOURCE)
#define _GNU_SOURCE
#endif
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PLUGIN_PATH "target/debug/libshadow_plugin_rust_test.so"
#define START_SYMBOL "start"

int main() {
  dlerror();
  void* handle = dlmopen(LM_ID_NEWLM, PLUGIN_PATH, RTLD_LAZY|RTLD_LOCAL);
  if(!handle) {
    printf("dlmopen failed\n");
    return EXIT_FAILURE;
  }
  dlerror();
  void (*fn)(int argc, char** argv) = dlsym(handle, START_SYMBOL);
  if(!fn) {
    printf("dlsym faild\n");
    dlclose(handle);
    return EXIT_FAILURE;
  }

  /* run the plugin */
  printf("running the plugin\n");
  fn(0, NULL);

  return EXIT_SUCCESS;
}
