from sys import argv
from functools import reduce
from operator import mul

# who doesn't love a nice, cryptic one-liner? (Readable version below)
print(reduce(lambda x,y:x*y,map(lambda x:int(argv[1])//x+1,map(int,argv[2:])),1))



#product = lambda xs: r(lambda x, y: x * y, xs, 1)
## or...
#def product(xs):
#    p = 1
#    for x in xs:
#        p *= x
#    return p
#
#price, stamps = int(argv[1]), [int(x) for x in argv[2:]]
#print(product((price // x) + 1 for x in stamps))
