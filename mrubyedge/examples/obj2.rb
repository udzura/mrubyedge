class OhMyClass
  def initialize
    @value = 0
  end

  def update(v)
    @value = v
  end

  def value
    @value
  end

  def hello
    puts "The value = #{value}"
    value * 2
  end
end

def main
  obj = OhMyClass.new
  debug obj
  obj.update(123)
  obj.hello
end

main