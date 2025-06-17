from .jposit import from_bits, from_double, get_version  # Rust functions

__all__ = ['from_bits', '__version__', 'demo']

__version__ = get_version()

def demo():
  print (f"{from_bits(123, 8, 0)=}")
  print (f"{from_bits(123, 8, 5)=}")
  print (f"{from_bits(123, 16, 1)=}")
  print (f"{from_double(44.3, 16, 1)=}")
  print (f"{from_double(44.3, 16, 6)=}")
