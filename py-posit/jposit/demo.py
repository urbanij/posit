def demo():
  from . import from_bits, from_double
  print (f"{from_bits(123, 8, 0)=}")
  print (f"{from_bits(123, 8, 5)=}")
  print (f"{from_bits(123, 16, 1)=}")
  print (f"{from_double(44.3, 16, 1)=}")
  print (f"{from_double(44.3, 16, 6)=}")
