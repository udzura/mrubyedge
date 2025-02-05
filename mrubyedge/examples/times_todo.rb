# TODO: This won't work using shifted register slice.
# if we want to allow orphaned lambda,
# we need to use an environment structure to capture the upvalues. 
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