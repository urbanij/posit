typedef unsigned int u32;
typedef unsigned long int u64;
typedef int i32;
typedef double f64;

typedef struct Posit Posit;


typedef struct Posit Posit;
struct Posit {
  u64 bits;
};

typedef struct rust__f64 rust__f64;
struct rust__f64 {
  f64 __value;
};

//Posit* from_bits(u64, u32, u32);
rust__f64* from_bits(u64, u32, u32);



#if 0
/*************************/
typedef struct Foo Foo;
struct Foo {
  i32 a;
};

Foo* get_foo();
/*************************/
#endif