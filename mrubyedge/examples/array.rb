ary = []
ary2 = [1, 2, 3]
ary = ary + ary2
debug ary[0]

ha = {}
ha[:a] = 1
ha2 = {b: 2, c: 3}
ha = ha.merge(ha2)
debug ha[:a]
ha[:a] = 100
debug ha[:a]
