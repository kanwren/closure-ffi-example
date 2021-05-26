#include "example.h"

/**
 * Takes a function pointer and calls it on the provided argument
 */
void c_call(void (*fun)(void *), void *arg) {
    fun(arg);
}

void (*deferred_fun)(void *);
void *deferred_fun_arg;

/**
 * Takes a function pointer and stores it to be called later.
 */
void c_deferred_call(void (*fun)(void *), void *arg) {
    deferred_fun = fun;
    deferred_fun_arg = arg;
}

/**
 * Takes the stored function pointer and calls it
 */
void c_do_deferred_call(void) {
    deferred_fun(deferred_fun_arg);
}

