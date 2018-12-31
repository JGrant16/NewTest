#valid = "^((([a-zA-Z0-9_])|(\-)|(\.)|(\/))+$)"    (\-)+(\.)+(\/)+
#valid = "^[\.\/A-Za-z0-9_-]+$"
#t = "from: 3 to: 7,9,2,10"
#t =~ /^from: ([\-]{0}[0-9]+) to: ([\-]{0}[\,0-9]+)$/
#puts $1
#t =~ /^from: ([\-]{0}[0-9]+) to: ([\-]{0}[\,0-9]+)$/
#puts $1
#okay
#something new


#p = "adasdload"
#p =~ /^[\ ]*(load[a-z]*)/
#if ($1 != nil) then
      #      puts $1
#end
require "securerandom"
salt = (0...30).map { (65 + rand(26)).chr }.join
new = "rustyremy" << salt;
puts salt
puts Digest::SHA256.hexdigest new

word = "he\llo"
if word =~ /[-!$%^&*()_+|~={}\:;<>?,.\/]/ then
    puts $1
end
word = "<<ff>>'''4"
if (word.include?("z")) then
    puts "yes"
end

def remove_bad(input)
    input = input.gsub(/<[^>]*>*/, '').chomp
    puts input
end
remove_bad(word)
word = "hello"
if (word =~ (/([\w]$)/)) then
    puts "yes"
    puts $1
end
