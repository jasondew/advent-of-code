require "digest/md5"

hashes = Hash.new {|h, k| h[k] = digest(k) }
keys_found = 0

def digest(index)
  prefix = "yjdafjpo"
  digest = Digest::MD5.hexdigest "#{prefix}#{index}"
  2016.times { digest = Digest::MD5.hexdigest digest }
  digest
end

(0..1_000_000).each do |index|
  break if keys_found == 64
  if match = hashes[index].scan(/(.)\1{2}/).first
    if ((index + 1)..(index + 1_000)).detect {|i| hashes[i].scan(/#{match.first}{5}/).first }
      keys_found += 1
      puts "key #{keys_found} found! #{index}"
    end
  end
end
