def times_break
  5.times do |i|
    puts "loop #{i}"
    if i > 3
      break 100
    end
  end
end

def times_break_2
  5.times do |i|
    if i > 3
      next
    end
    puts "loop #{i}"
  end
end

def while_break
  i = 0
  while true
    puts "loop #{i}"
    i += 1
    if i > 3
      break 100
    end
  end
end

def while_break_2
  i = 0
  while true
    puts "loop #{i}"
    i += 1
    if i <= 3
      next
    end
    return
  end
end