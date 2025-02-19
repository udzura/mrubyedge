def do_rescue
  begin
    raise "An error occurred"
    puts "all is OK"
  rescue HogeError => e
    puts "Rescued: #{e.message}"
  rescue => e
    puts "Rescued: #{e.message}"
  end
end