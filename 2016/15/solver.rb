def d1(n); (n+1) % 17; end
def d2(n); n % 7; end
def d3(n); (n + 2) % 19; end
def d4(n); n % 5; end
def d5(n); n % 3; end
def d6(n); (n + 5) % 13; end
def d7(n); n % 11; end

p (0..1_000_000_000).detect {|t| d1(t+1) + d2(t+2) + d3(t+3) + d4(t+4) + d5(t+5) + d6(t+6) + d7(t+7) == 0 }
