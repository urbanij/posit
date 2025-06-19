import math
from .jposit import __from_bits as __posit_from_bits
from .jposit import __from_double as __posit_from_double
from .jposit import __get_version
from .demo import demo

__all__ = ['from_bits', 'from_double']

__version__ = __get_version()

def from_bits(bits, n, es):
  ans = __posit_from_bits(bits, n, es)
  return None if math.isnan(ans) else ans

def from_double(x, n, es):
  if math.isnan(x):
    return math.inf
  ans = __posit_from_double(x, n, es)
  return None if math.isnan(ans) else ans
