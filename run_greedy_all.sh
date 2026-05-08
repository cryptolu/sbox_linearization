#!/bin/bash

mkdir logs_custom logs_sage logs_super logs_monomial

ls -1 sboxes_custom/n*               | parallel --line-buffer -j 16 bash run_greedy_sbox.sh 300 logs_custom
ls -1 sboxes_sage/n*                 | parallel --line-buffer -j 16 bash run_greedy_sbox.sh 300 logs_sage
ls -1 sboxes_super/n16_*             | parallel --line-buffer -j 2  bash run_greedy_sbox.sh 14400 logs_super

ls -1 sboxes_monomial/n{4,5,6,7,8}_* | parallel --line-buffer -j 16 bash run_greedy_sbox.sh 300   logs_monomial
ls -1 sboxes_monomial/n{9,10,11}_*   | parallel --line-buffer       bash run_greedy_sbox.sh 3600  logs_monomial
ls -1 sboxes_monomial/n{12,13}_*     | parallel --line-buffer       bash run_greedy_sbox.sh 7200  logs_monomial
ls -1 sboxes_monomial/n14_*          | parallel --line-buffer       bash run_greedy_sbox.sh 7200  logs_monomial
ls -1 sboxes_monomial/n15_*          | parallel --line-buffer       bash run_greedy_sbox.sh 7200  logs_monomial
ls -1 sboxes_monomial/n16_*          | parallel --line-buffer -j 2  bash run_greedy_sbox.sh 14400 logs_monomial

# DDT n=14 Alberti: 44s 10.5GB
# DDT n=15 Alberti: 226s 42GB
# DDT n=16 Alberti: 1015s 168GB