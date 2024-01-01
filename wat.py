import re

splits = {c: [0, 4000] for c in 'xmas'}

flows = "lrf{s>2166:A,m<1261:nfs,R}"

for c,o,v in re.findall(r'(\w+)(<|>)(\d+)', flows):
    splits[c].append(int(v)-(o=='<'))

print(splits)

ranges = lambda x: [(a,a-b) for a,b in zip(x[1:], x)]
X,M,A,S = [ranges(sorted(splits[x])) for x in splits]

print(M)

C = 0
for x,dx in X:
    for m,dm in M:
        for a,da in A:
            for s,ds in S:
                pass
                # C += dx * dm * da * ds * bool(in_()-1)

print(C)