import math
from . import _native
import cffi

ffi = cffi.FFI()

_posit_from_bits = _native.lib.from_bits
_get_version = _native.lib.get_version


__version__ = ffi.string(_get_version()).decode('utf-8')

__all__ = ['from_bits', '__version__', 'demo']


def from_bits(bits, n, es):
  ans = _posit_from_bits(bits, n, es).__value
  return None if math.isnan(ans) else ans


def demo():
  print (from_bits(123, 8, 0))
  print (from_bits(123, 8, 5))
  print (from_bits(123, 16, 1))
