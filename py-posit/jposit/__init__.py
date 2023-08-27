import math
from . import _native

_posit_from_bits = _native.lib.from_bits
__version__ = 0 # TODO: override cargo version

def from_bits(bits, n, es):
  ans = _posit_from_bits(bits, n, es).__value
  return None if math.isnan(ans) else ans


def demo():
  print (from_bits(123, 8, 0))
  print (from_bits(123, 8, 5))
  print (from_bits(123, 16, 1))
