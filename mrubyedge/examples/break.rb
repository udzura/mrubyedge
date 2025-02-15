def times_break
  5.times do |i|
    puts "loop #{i}"
    if i > 3
      break
    end
  end
end

def while_break
  i = 0
  while true
    puts "loop #{i}"
    i += 1
    if i > 3
      break
    end
  end
end