class MyMRubyClass
  attr_accessor :value

  def print_self
    puts "Value: #{value}"
  end
end

def __main__
  obj = MyMRubyClass.new
  obj.value = 456
  obj.print_self
  obj.value = 5471
  obj.print_self
  obj.value = 557188
  obj.print_self
  return obj.value
end
