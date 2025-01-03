class MyMRubyClass
  def initialize(value)
    @value = value
  end

  def update(value)
    @value = value * 2
  end

  def value
    @value
  end

  def print_self
    puts "Value: #{value}"
  end
end

def main
  obj = MyMRubyClass.new(123)
  obj.print_self
  obj.update(456)
  obj.print_self
  return obj.value
end