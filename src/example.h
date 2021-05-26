#ifndef EXAMPLE_H_
#define EXAMPLE_H_

void c_call(void (*fun)(void *), void *arg);

void c_deferred_call(void (*fun)(void *), void *arg);
void c_do_deferred_call(void);

#endif // EXAMPLE_H_
