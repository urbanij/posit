#include <stdint.h>

typedef unsigned int u32;
typedef unsigned long int u64;
typedef int i32;
typedef double f64;

/// Get version
char* get_version();

/// Posit
typedef struct {
  u64 bits;
} Posit;

/// 
typedef struct {
  f64 __value;
} rust__f64;


//Posit* from_bits(u64, u32, u32);
rust__f64* from_bits(u64, u32, u32);

// typedef struct {
//     int32_t is_error;
//     union {
//         rust__f64* value;
//         void* error;
//     } data;
// } RustF64Result;

rust__f64* from_double(f64, u32, u32);


#if 0
/*************************/
typedef struct Foo Foo;
struct Foo {
  i32 a;
};

Foo* get_foo();
/*************************/
#endif



/*

import jposit
p = jposit._posit_from_double(1.2, 16, 1)
p.data

*/

