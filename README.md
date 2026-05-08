# Algorithmic Toolkit for Linearization of S-boxes

This repository contains supporting code for the [EUROCRYPT 2026](https://eurocrypt.iacr.org/2026/) paper ([doi](https://doi.org/10.1007/978-3-032-25333-0_9)), ([eprint full version](https://eprint.iacr.org/2026/xx)):

> **Algorithmic Toolkit for Linearization of S-boxes**

by Alex Biryukov, Philip Tureček, Aleksei Udovenko.

The work was funded by Luxembourg's FNR projects CryptoFin (C22/IS/17415825) and PQseal (C24/IS/18978392).

```bib
@InProceedings{EC:BirTurUdo26,
    author="Biryukov, Alex and Ture{\v{c}}ek, Philip and Udovenko, Aleksei",
    editor="Daemen, Joan and Thom{\'e}, Emmanuel",
    title="Algorithmic Toolkit for Linearization of S-Boxes",
    booktitle="Advances in Cryptology -- EUROCRYPT 2026",
    year="2026",
    publisher="Springer Nature Switzerland",
    address="Cham",
    pages="243--272",
    isbn="978-3-032-25333-0",
    doi="10.1007/978-3-032-25333-0_9"
}

```

A copy of this repository is available at [zenodo.org](https://doi.org/10.5281/zenodo.18715466).

A vibe-coded interactive visualization of the GreedyExtension(DDT) algorithm is available at [affine.group/pages/greedy-extension](https://affine.group/pages/greedy-extension)and [./interactive-greedy-extension-AI.html](./interactive-greedy-extension-AI.html).


## S-box file format

The programs take as input an S-box file which have format

```
n m
S[0] S[1] S[2] ... S[2^n-1]
```

For example, 

```
8 8
101 76 106 ... 255
```

In this repository, the S-box files are stored compressed with bzip2. You can run the algorithms by piping as `bzcat sbox-file.txt.bz2 | program` .

Generally, there are 4 classes of S-boxes considered: `sage` (available in `sage.crypto.sbox.sboxes`, mainly crypto S-boxes), `monomial`, `super` (SuperSboxes), `custom` (some other examples). They are available in the following folders:

```
sboxes_sage/
sboxes_monomial/
sboxes_super/
sboxes_custom/
```

## Results and logs

For all S-boxes in the repository, resulting approximations with extra informations are available in files like

```bash
$ cat sboxes_sage/n8_AES.sol.18.txt 
0 20 63 89 119 122 143 149 160 169 177 185 191 196 227 253 254 255
Iter#2282363 New set #3 of size 18 (760787.7 its/hit = 4.000s/hit): seed=3926565703075128789 [0, 20, 63, 89, 119, 122, 143, 149, 160, 169, 177, 185, 191, 196, 227, 253, 254, 255]
logs_sage/n8_AES.txt.bz2.greedyext.20260220_154050.txt

0 2 16 58 77 94 106 108 114 125 128 148 159 189 197 198 203 204
Iter#27690644 New set #40 of size 18 (692266.1 its/hit = 3.800s/hit): seed=13025246629886185991 [0, 2, 16, 58, 77, 94, 106, 108, 114, 125, 128, 148, 159, 189, 197, 198, 203, 204]
logs_sage/n8_AES.txt.bz2.greedyext.20260220_154050.txt
...
```

Here, each block contains the matching inputs of the approximation, a log line that produced this approximation, and the corresponding log file.

Logs are available in the following folders:

```
logs_custom/
logs_monomial/
logs_sage/
logs_super/
````

Minimal summaries for each category are available in files

```
summary_logs_custom.txt
summary_logs_monomial.txt
summary_logs_sage.txt
summary_logs_super.txt
```

```bash
$ tail summary_logs_sage.txt
logs_sage/n8_Streebog.txt.bz2.greedyext.20260220_155550.txt | size 20 
logs_sage/n8_Stribog.txt.bz2.greedyext.20260220_155550.txt | size 20 
logs_sage/n8_Turing.txt.bz2.greedyext.20260220_155550.txt | size 21 
logs_sage/n8_Twofish_p0.txt.bz2.greedyext.20260220_155550.txt | size 22 
logs_sage/n8_Twofish_p1.txt.bz2.greedyext.20260220_155550.txt | size 22 
logs_sage/n8_Whirlpool.txt.bz2.greedyext.20260220_155550.txt | size 21 
logs_sage/n8_Zorro.txt.bz2.greedyext.20260220_155550.txt | size 21 
logs_sage/n8_ZUC_S0.txt.bz2.greedyext.20260220_155550.txt | size 23 
logs_sage/n8_ZUC_S1.txt.bz2.greedyext.20260220_155550.txt | size 18 
logs_sage/n9_DryGASCON256.txt.bz2.greedyext.20260220_155550.txt | size 77 
```


## Greedy extension algorithm (DDT-related), heuristic

The first algorithm is the fastest and produces the best results. It is implemented in Rust.

This version is limited to 16-bit S-boxes (inputs and outputs) for optimization purposes. In principle, supporting larger **output** sizes is not a problem; supporing larger **input** sizes is not feasible due to DDT precomputation.

Compilation:

```bash
$ (cd greedy_extension && cargo build --release)
```

Execution:

```bash
bzcat sboxes_sage/n8_SKINNY_8.txt.bz2 | ./greedy_extension/target/release/greedy_extention 100  # optional parameter: number of iterations
[*] Computing DDT
[+] DDT Computed
DDT took 1.462055ms
Iter#1 New set #1 of size 45 (1.0 its/hit = 0.000s/hit): seed=15234621534090088038 [67, 70, 76, 83, 85, 86, 92, 115, 117, 118, 124, 131, 132, 134, 140, 142, 178, 179, 180, 181, 182, 183, 188, 190, 195, 196, 198, 204, 206, 210, 211, 212, 213, 214, 215, 220, 222, 242, 243, 244, 245, 246, 247, 252, 254]
Iter#3 New set #2 of size 45 (1.5 its/hit = 0.000s/hit): seed=9903632497483712326 [0, 8, 9, 12, 13, 16, 25, 29, 32, 40, 41, 64, 66, 73, 75, 77, 80, 82, 88, 89, 90, 91, 92, 93, 114, 122, 123, 124, 125, 128, 130, 137, 139, 141, 144, 146, 152, 153, 154, 155, 156, 157, 176, 184, 185]
...
Iter#71 New set #10 of size 47 (7.1 its/hit = 0.000s/hit): seed=14806622087529903567 [0, 8, 9, 32, 34, 40, 41, 42, 43, 46, 47, 48, 50, 57, 59, 63, 82, 90, 91, 94, 95, 96, 98, 105, 107, 111, 112, 114, 120, 121, 122, 123, 126, 127, 210, 218, 219, 222, 223, 226, 235, 239, 242, 250, 251, 254, 255]
```

The output lists correspond to the matching input points of the approximation.


## Exhaustive reduction algorithm (LAT-related)

The second algorithm is slower but produces guaranteed results. It is implemented in C++ and has a similar interface.

Compilation:

```bash
$ (cd exhaustive_reduction && make)
```

It produces 2 programs: ReductionExhaustive8 and ReductionExhaustive16, for up to 8- and 16-bit S-boxes respectively.

Execution:

```bash
bzcat sboxes_sage/n8_SKINNY_8.txt.bz2 | ./greedy_extension/target/release/greedy_extention 47  # lower bound g (LB)
Input S-box (n m then space separated integers):
n = 8, m = 8, 2^n = 256
101 76 106 66 75 99 67 107 85 117 90 122 83 115 91 123 53 140 58 129 137 51 128 59 149 37 152 42 144 35 153 43 229 204 232 193 201 224 192 233 213 245 216 248 208 240 217 249 165 28 168 18 27 160 19 169 5 181 10 184 3 176 11 185 50 136 60 133 141 52 132 61 145 34 156 44 148 36 157 45 98 74 108 69 77 100 68 109 82 114 92 124 84 116 93 125 161 26 172 21 29 164 20 173 2 177 12 188 4 180 13 189 225 200 236 197 205 228 196 237 209 241 220 252 212 244 221 253 54 142 56 130 139 48 131 57 150 38 154 40 147 32 155 41 102 78 104 65 73 96 64 105 86 118 88 120 80 112 89 121 166 30 170 17 25 163 16 171 6 182 8 186 0 179 9 187 230 206 234 194 203 227 195 235 214 246 218 250 211 243 219 251 49 138 62 134 143 55 135 63 146 33 158 46 151 39 159 47 97 72 110 70 79 103 71 111 81 113 94 126 87 119 95 127 162 24 174 22 31 167 23 175 1 178 14 190 7 183 15 191 226 202 238 198 207 231 199 239 210 242 222 254 215 247 223 255 
S-box is quadratic? 0

RUNNING WITH LB 47, BATCH index=0 (1/1)
Options: FULL_STAT 0, X_SPLIT_TESTING 1
checking masks itr=    2341 (83 e8) thread 1/28
checking masks itr=   63205 (3e 4f) thread 27/28
checking masks itr=   37472 (ed 87) thread 16/28
checking masks itr=   16417 (d6 4c) thread 7/28
checking masks itr=   37494 (a8 b5) thread 16/28
...
SOLUTION depth 8: 47 : 37 39 42 43 44 45 46 47 53 55 59 61 63 85 87 90 91 92 93 94 95 101 103 107 109 111 117 119 122 123 124 125 126 127 215 218 219 222 223 231 235 239 247 250 251 254 255 
SOLUTION depth 8: 47 : 26 28 30 37 39 43 45 47 53 55 59 61 63 69 71 74 75 76 77 78 79 101 103 106 107 108 109 110 111 117 119 123 125 127 199 202 203 206 207 231 234 235 238 239 247 251 255 
SOLUTION depth 8: 47 : 0 8 9 32 34 40 41 42 43 46 47 48 50 57 59 63 82 90 91 94 95 96 98 105 107 111 112 114 120 121 122 123 126 127 210 218 219 222 223 226 235 239 242 250 251 254 255 
SOLUTION depth 8: 47 : 16 24 25 26 30 32 34 41 43 47 48 50 56 57 59 63 66 72 74 75 78 79 96 98 105 106 107 110 111 112 114 121 123 127 194 202 203 206 207 226 234 235 238 239 242 251 255 
SOLUTION depth 8: 47 : 4 10 11 14 15 36 38 42 43 44 45 46 47 52 58 62 84 86 90 91 92 93 94 95 100 102 106 108 110 148 150 154 155 156 157 158 159 164 166 170 172 174 180 186 187 190 191 
...
SOLUTION depth 8: 47 : 87 90 91 94 95 103 107 111 119 122 123 126 127 165 167 171 173 175 181 183 186 187 188 189 190 191 213 215 218 219 220 221 222 223 229 231 235 237 239 245 247 250 251 252 253 254 255 
SOLUTION depth 8: 47 : 71 74 75 78 79 103 106 107 110 111 119 123 127 138 140 142 165 167 171 173 175 181 183 187 189 191 197 199 202 203 204 205 206 207 229 231 234 235 236 237 238 239 245 247 251 253 255 
SOLUTION depth 8: 47 : 68 74 78 84 90 91 94 95 116 122 123 126 127 132 134 138 140 142 180 182 186 187 188 189 190 191 196 198 202 204 206 212 214 218 219 220 221 222 223 244 246 250 251 252 253 254 255 
SOLUTION depth 8: 47 : 68 74 75 78 79 100 106 107 110 111 116 122 126 132 134 138 140 142 171 173 175 180 182 186 188 190 196 198 202 203 204 205 206 207 228 230 234 235 236 237 238 239 244 246 250 252 254 
SOLUTION depth 8: 47 : 82 90 91 94 95 98 107 111 114 122 123 126 127 144 152 153 160 162 169 171 175 176 178 184 185 186 187 190 191 210 218 219 222 223 224 226 233 235 239 240 242 248 249 250 251 254 255 
SOLUTION depth 8: 47 : 66 74 75 78 79 98 106 107 110 111 114 123 127 128 136 137 138 142 160 162 168 169 171 175 176 178 185 187 191 194 200 202 203 206 207 224 226 233 234 235 238 239 240 242 249 251 255 
SOLUTION depth 8: 47 : 20 22 26 27 28 30 31 36 38 42 44 46 52 58 59 61 62 63 68 70 74 76 78 107 109 111 116 118 122 124 126 132 138 139 141 142 143 164 166 170 171 172 174 175 180 186 190 
SOLUTION depth 8: 47 : 4 6 10 11 12 13 14 15 20 26 30 36 42 43 46 47 132 134 138 140 142 148 154 155 158 159 180 182 186 187 188 189 190 191 196 198 202 204 206 244 246 250 251 252 253 254 255 
SOLUTION depth 8: 47 : 7 11 15 23 26 27 28 30 31 53 55 58 59 61 62 63 69 71 75 77 79 85 87 91 93 95 106 108 110 133 135 138 139 141 142 143 149 151 155 157 159 167 170 171 172 174 175 
...

depth 0:             0 = 2^  -inf nodes,             0 = 2^  -inf leaves,             0 = 2^  -inf non-leaves (-nan %)
depth 1:             0 = 2^  -inf nodes,             0 = 2^  -inf leaves,             0 = 2^  -inf non-leaves (-nan %)
depth 2:        262144 = 2^ 18.00 nodes,          3574 = 2^ 11.80 leaves,        258570 = 2^ 17.98 non-leaves (98.64 %)
depth 3:     132387840 = 2^ 26.98 nodes,     130574234 = 2^ 26.96 leaves,       1813606 = 2^ 20.79 non-leaves (1.37 %)
depth 4:     928566272 = 2^ 29.79 nodes,     928130660 = 2^ 29.79 leaves,        435612 = 2^ 18.73 non-leaves (0.05 %)
depth 5:     223033344 = 2^ 27.73 nodes,     222997878 = 2^ 27.73 leaves,         35466 = 2^ 15.11 non-leaves (0.02 %)
depth 6:      18158592 = 2^ 24.11 nodes,      18150272 = 2^ 24.11 leaves,          8320 = 2^ 13.02 non-leaves (0.05 %)
depth 7:       4259840 = 2^ 22.02 nodes,       4259148 = 2^ 22.02 leaves,           692 = 2^  9.43 non-leaves (0.02 %)
depth 8:        354304 = 2^ 18.43 nodes,        354264 = 2^ 18.43 leaves,            40 = 2^  5.32 non-leaves (0.01 %)
depth 9:             0 = 2^  -inf nodes,             0 = 2^  -inf leaves,             0 = 2^  -inf non-leaves (-nan %)
  total:    1307022336 = 2^ 30.28 nodes,    1304470030 = 2^ 30.28 leaves
  prefix   skips: 0 = 2^  -inf
  prefix checked: 65536 = 2^ 16.00
main loop prefixes checked: 65536
Finished
```

For highly parallel environments, it is possible to split the work in batches:

```
bzcat sboxes_sage/n8_SKINNY_8.txt.bz2 | ./greedy_extension/target/release/greedy_extention 47 0 4  # LB , batch index , #batches
bzcat sboxes_sage/n8_SKINNY_8.txt.bz2 | ./greedy_extension/target/release/greedy_extention 47 1 4
bzcat sboxes_sage/n8_SKINNY_8.txt.bz2 | ./greedy_extension/target/release/greedy_extention 47 2 4
bzcat sboxes_sage/n8_SKINNY_8.txt.bz2 | ./greedy_extension/target/release/greedy_extention 47 3 4
```


## Application to CICO

The notebook [CoverageCICO.ipynb](./CoverageCICO.ipynb) contains implementation of the probabilistic domain covering technique, as well as CICO solving.