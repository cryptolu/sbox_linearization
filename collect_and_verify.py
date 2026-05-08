from sage.all import matrix, vector, GF
from binteger import Bin

import os, ast, bz2
from glob import glob
from collections import defaultdict, namedtuple


def affine_rank(xs, n):
    res = [
        Bin.concat(Bin(x, n), 1).vector
        for x in xs
    ]
    return matrix(GF(2), res).rank() - 1


def verify(X, S, n, m):
    assert n == m, "not tested for n != m"
    xs = []
    xys = []
    for x in X:
        assert 0 <= x < 2**n
        y = S[x]
        assert 0 <= y < 2**m
        xs.append( x )
        xys.append( (y << n) | x )
    rank_xs = affine_rank(xs, n)
    rank_xys = affine_rank(xys, n + m)
    if rank_xys > rank_xs:
        raise ValueError("Affine approximation is inconsistent!")
    if rank_xs != n:
        raise ValueError("Affine approximation is incomplete!")
    return True


Sbox = namedtuple("Sbox", ["dir", "file", "n", "m", "sbox"])

sboxes = {}
for sboxfile in glob("sboxes_*/*.txt.bz2"):
    name = os.path.basename(sboxfile).split(".")[0]
    assert name not in sboxes, (sboxfile, sboxes[name])

    print("Reading sbox", name, "...")
    if sboxfile.endswith(".txt"):
        f = open(sboxfile)
    elif sboxfile.endswith(".txt.bz2"):
        f = bz2.open(sboxfile)
    else:
        assert 0, sboxfile

    with f as f:
        n, m = map(int, f.readline().split())
        sbox = tuple(map(int, f.readline().split()))
        assert len(sbox) == 2**n, (sboxfile, n, m, len(sbox))
        assert 0 <= min(sbox) <= max(sbox) < 2**m

    sboxes[name] = Sbox(os.path.dirname(sboxfile), sboxfile, n, m, sbox)
print()


bests = defaultdict(lambda: (-1, []))
for logfile in sorted(glob("logs_*/*.txt")):
    name = os.path.basename(logfile).split(".")[0]
    assert name in sboxes, (name, logfile)
    sbox = sboxes[name]
    n = sbox.n
    m = sbox.m
    S = sbox.sbox
    print("Log for sbox", name)

    for line in open(logfile):
        if " [" not in line:
            continue
        line = line.strip()
        parts = line.split("[")[-1].strip().strip("]")
        X = ast.literal_eval(parts)
        sz = len(X)
        if sz < bests[name][0]:
            continue
        if sz > bests[name][0]:
            bests[name] = sz, set(), {}
        bests[name][1].add(X)
        bests[name][2][X] = logfile, line
print()


for name in sorted(bests):
    sz, Xs, info = bests[name]

    sbox = sboxes[name]
    n = sbox.n
    m = sbox.m
    S = sbox.sbox
    sbox_dir = sbox.dir

    print("Solutions for", name)
    print("  size", sz, ":", len(Xs), "approximations")
    for X in Xs:
        assert len(X) == sz
        assert verify(X, S, n, m)

    solfile = os.path.join(sbox_dir, name + ".sol.%d.txt" % len(X))
    with open(solfile, "w") as f:
        for X in Xs:
            logfile, line = info[X]
            print(*sorted(X), file=f)
            print(line, file=f)
            print(logfile, file=f)
            print(file=f)




#Iter#17456368 New set #2 of size 42 (8728184.0 its/hit = 2333.000s/hit): seed=8195996954938249473 [0, 136, 199, 240, 256, 936, 947, 951, 974, 1193, 1229, 1244, 1267, 1269, 1470, 1490, 1574, 1585, 1788, 2032, 2080, 2157, 2233, 2248, 2315, 2539, 2698, 2776, 2924, 2994, 3036, 3087, 3100, 3135, 3144, 3498, 3617, 3651, 3703, 3834, 4052, 4056]
