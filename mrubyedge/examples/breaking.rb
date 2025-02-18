def onetimes
  yield
  puts "whoa?"
end

onetimes do
  puts "dummy"
  break
end