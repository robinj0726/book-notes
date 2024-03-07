class Node:
  def __init__(self, val, rest=None):
    self.value = val
    self.next = rest

def sum_iterative(n):
  total = 0                         
  while n:
    total += n.value                
    n = n.next                      
  return total

def sum_list(n):
  # Base case: the sum of a nonexistent list is 0.
  if n is None:                     
    return 0
  return n.value + sum_list(n.next) 