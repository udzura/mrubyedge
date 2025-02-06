def do_time_block
  result = 0
  ->(i) {
    result += 100
    puts "result = #{result}"
  }
end

def do_times
  3.times(&do_time_block)
end

do_times