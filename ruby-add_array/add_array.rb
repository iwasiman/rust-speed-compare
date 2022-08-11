=begin
$ ruby -v
ruby 2.6.8p205 (2021-07-07 revision 67951) [universal.x86_64-darwin21]

$ time ruby add_array.rb 10000 10000
Rubyでの実行結果100000000
real    0m6.862s
user    0m6.731s
sys     0m0.040s

$ time ruby add_array.rb 100000 100000
Rubyでの実行結果10000000000
real    11m26.039s
user    11m8.511s
sys     0m1.403s

=end

def add_array(n, x)
    a = Array.new(n, 0)
    x.times do
        for i in 0..x-1
            a[i]  += 1
        end
    end
    a.sum
end
print "Rubyでの実行結果", add_array(ARGV[0].to_i, ARGV[1].to_i)