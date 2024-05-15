def fib(n)
  if n < 1
    return 0
  elsif n < 3
    return 1
  else
    return fib(n-1)+fib(n-2)
  end
end

def bench(num)
  start = Time.now.to_i
  fib(num)
  fin = Time.now.to_i
  p (fin - start)
end
