#!/bin/bash

# LOGFOLDER="$1"
# SBOX_PATH="$2"
# MAXVAL="$3"

bash run_exhaustive_sbox.sh "$LOGFOLDER" "$SBOX_PATH" "$MAXVAL"

cat <<EOF
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_AES.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Anubis.txt.bz2,MAXVAL=21 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_ARIA_s2.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_BelT.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Camellia.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Chiasmus.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_CLEFIA_S0.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_CLEFIA_S1.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_CMEA.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Crypton_0_5.txt.bz2,MAXVAL=25 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Crypton_1_0_S0.txt.bz2,MAXVAL=24 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Crypton_1_0_S1.txt.bz2,MAXVAL=24 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Crypton_1_0_S2.txt.bz2,MAXVAL=24 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Crypton_1_0_S3.txt.bz2,MAXVAL=24 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_CSA.txt.bz2,MAXVAL=21 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_CS_cipher.txt.bz2,MAXVAL=29 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_CSS.txt.bz2,MAXVAL=81 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_DBlock.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_E2.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Enocoro.txt.bz2,MAXVAL=23 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Fantomas.txt.bz2,MAXVAL=28 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_FlexAEAD.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_FLY.txt.bz2,MAXVAL=31 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_ForkSkinny_8.txt.bz2,MAXVAL=47 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Fox.txt.bz2,MAXVAL=27 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Iceberg.txt.bz2,MAXVAL=21 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Iraqi.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_iScream.txt.bz2,MAXVAL=27 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Kalyna_pi0.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Kalyna_pi1.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Kalyna_pi2.txt.bz2,MAXVAL=19 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Kalyna_pi3.txt.bz2,MAXVAL=19 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Khazad.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Kuznechik.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Kuznyechik.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Lilliput_AE.txt.bz2,MAXVAL=23 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_MD2.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_newDES.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Picaro.txt.bz2,MAXVAL=19 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Remus_8.txt.bz2,MAXVAL=47 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Romulus.txt.bz2,MAXVAL=47 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Safer.txt.bz2,MAXVAL=30 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Scream.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_SEED_S0.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_SEED_S1.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_SKINNY_8.txt.bz2,MAXVAL=47 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Skipjack.txt.bz2,MAXVAL=21 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_SMS4.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_SNOW_3G_sq.txt.bz2,MAXVAL=26 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Streebog.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Stribog.txt.bz2,MAXVAL=20 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Turing.txt.bz2,MAXVAL=21 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Twofish_p0.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Twofish_p1.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Whirlpool.txt.bz2,MAXVAL=21 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_Zorro.txt.bz2,MAXVAL=21 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_ZUC_S0.txt.bz2,MAXVAL=23 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 16:00:00 --export=LOGFOLDER=logs_sage/,SBOX_PATH=sboxes_sage/n8_ZUC_S1.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_monomial/,SBOX_PATH=sboxes_monomial/n8_monomial3.txt.bz2,MAXVAL=16 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_monomial/,SBOX_PATH=sboxes_monomial/n8_monomial5.txt.bz2,MAXVAL=16 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_monomial/,SBOX_PATH=sboxes_monomial/n8_monomial7.txt.bz2,MAXVAL=22 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_monomial/,SBOX_PATH=sboxes_monomial/n8_monomial8.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_monomial/,SBOX_PATH=sboxes_monomial/n8_monomialSqrt.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_monomial/,SBOX_PATH=sboxes_monomial/n8_monomialInverse.txt.bz2,MAXVAL=18 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_custom/,SBOX_PATH=sboxes_custom/n8_LFT216.txt.bz2,MAXVAL=17 ./sbatcher.sh
sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=LOGFOLDER=logs_custom/,SBOX_PATH=sboxes_custom/n8_Wuhan3.txt.bz2,MAXVAL=17 ./sbatcher.sh
EOF