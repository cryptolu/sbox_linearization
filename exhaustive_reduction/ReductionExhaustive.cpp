#include <bits/stdc++.h>
#include <assert.h>
#include <omp.h>

using namespace std;

// enable for detailed report of split of set sizes across recursion levels
#define FULL_STAT 0

// test 2-pointer splitting
#define X_SPLIT_TESTING 1

// one node
// sbatch -N 1 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 1:00:00 --export=SBOX=aes,LB=18 ./batch.sh
// multiple nodes
// sbatch --tasks 8 -N 8 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=SBOX=apn8c1,LB=16 ./batch.sh
// sbatch --tasks 8 -N 8 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=SBOX=five8,LB=17 ./batch.sh
// sbatch --tasks 2 -N 2 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=SBOX=aes,LB=24 ./batch.sh
// sbatch --tasks 4 -N 4 --ntasks-per-node 1 --ntasks-per-socket 1 -c 128 -t 48:00:00 --export=SBOX=five9,LB=32 ./batch.sh
static vector<uint64_t> DERIVATIVES;

static int BITS;
static int NUM;

#ifdef WORD_U8
    typedef uint8_t word;
    static word S[256];
#endif
#ifdef WORD_U16
    typedef uint16_t word;
    static word S[65536];
#endif

static int LB;

// Class for collecting and reporting statistics (visited leaves, etc.)
struct STAT {
    uint64_t max_depth;
    vector<uint64_t> nodes;
    vector<uint64_t> leaves;
    uint64_t prefix_skips;

    #if FULL_STAT == 1
    vector<vector<uint64_t>> nodes_by_size;
    vector<vector<uint64_t>> leaves_by_size;
    #endif

    STAT(uint64_t _max_depth) : max_depth(_max_depth), prefix_skips(0) {
        nodes.resize(_max_depth + 1);
        leaves.resize(_max_depth + 1);
        #if FULL_STAT == 1
        nodes_by_size.assign(_max_depth + 1, vector<uint64_t>(NUM+1));
        leaves_by_size.assign(_max_depth + 1, vector<uint64_t>(NUM+1));
        #endif
    }
    void add(const STAT &cur_stat) {
        for(int depth = 0; depth <= max_depth; depth++) {
            nodes[depth] += cur_stat.nodes[depth];
            leaves[depth] += cur_stat.leaves[depth];
            #if FULL_STAT == 1
            for(int x = 0; x <= NUM; x++) {
                nodes_by_size[depth][x] += cur_stat.nodes_by_size[depth][x];
                leaves_by_size[depth][x] += cur_stat.leaves_by_size[depth][x];
            }
            #endif
        }
        prefix_skips += cur_stat.prefix_skips;
    }
    void report() const {
        uint64_t total_nodes = 0;
        uint64_t total_leaves = 0;
        for(int depth = 0; depth <= max_depth; depth++) {
            total_nodes += nodes[depth];
            total_leaves += leaves[depth];
            printf("depth %d:  %12lu = 2^%6.2lf nodes,  %12lu = 2^%6.2lf leaves,  %12lu = 2^%6.2f non-leaves (%.2f %%)\n",
                depth,
                nodes[depth], log(nodes[depth])/log(2),
                leaves[depth], log(leaves[depth])/log(2),
                (nodes[depth]-leaves[depth]), log(nodes[depth]-leaves[depth])/log(2),
                (nodes[depth]-leaves[depth]) * 100.0 / nodes[depth]
            );
            #if FULL_STAT == 1
            printf("  ");
            uint64_t total = 0;
            for(int x = 0; x <= NUM/4; x++) {
                auto num = nodes_by_size[depth][x];
                total += num;
                if (num == 0) {
                    printf("- ");
                }
                else {
                    printf("%5.2lf ", log(num)/log(2));
                }
                if (x % 16 == 15) printf("|\n  ");
            }
            printf("\n");
            printf("FULL STAT total: %lu = %.2lf\n", total, log(total)/log(2));
            #endif
        }
        printf("  total:  %12lu = 2^%6.2lf nodes,  %12lu = 2^%6.2lf leaves\n",
            total_nodes, log(total_nodes)/log(2),
            total_leaves, log(total_leaves)/log(2)
        );
        printf("  prefix   skips: %lu = 2^%6.2lf\n", prefix_skips, log(prefix_skips)/log(2));
        printf("  prefix checked: %lu = 2^%6.2lf\n", (1 << (2*BITS)) - prefix_skips, log((1 << (2*BITS)) - prefix_skips)/log(2));
    }
};

static void recurse(int depth, word * X, uint64_t Xsize, STAT &stat);

// Algorithm entry point: start with two fixed masks / bits
// (needed for parallelization purposes)
static void recurse_start(
    word mask1, uint8_t const1, word mask2, uint8_t const2,
    STAT &stat
) {

    // initialize the set X from the first two masks and constant bits
    vector<word> X;
    for (uint64_t x = 0; x < NUM; x++) {
        uint8_t ybit1 = (S[x] >> 0) & 1;
        uint8_t xbit1 = __builtin_popcount(x & mask1) & 1; // SCALAR[x][mask]
        if ( (xbit1 ^ ybit1) != const1 )
            continue;
        uint8_t ybit2 = (S[x] >> 1) & 1;
        uint8_t xbit2 = __builtin_popcount(x & mask2) & 1; // SCALAR[x][mask]
        if ( (xbit2 ^ ybit2) != const2 )
            continue;

        X.push_back(x);
    }
    // run main recursion
    recurse(2, X.data(), X.size(), stat);
}

// Main recursion function
static void recurse(int depth, word * X, uint64_t Xsize, STAT &stat) {
    // 0. Statistics collection
    stat.nodes[depth]++;
    #if FULL_STAT == 1
    stat.nodes_by_size[depth][Xsize]++;
    #endif

    // 1. Cut-off by LB
    if (Xsize < LB) {
        stat.leaves[depth]++;
        #if FULL_STAT == 1
        stat.leaves_by_size[depth][Xsize]++;
        #endif
        return;
    }

    // 2. Finished recursion? Report approximation.
    if (depth == BITS) {
        #pragma omp critical
        {
            printf("SOLUTION depth %d: %lu : ", depth, Xsize);
            sort(X, X + Xsize);
            for(size_t i = 0; i < Xsize; i++) {
                printf("%lu ", X[i]);
            }
            printf("\n");
            fflush(stdout);
        }
        return;
    }

    // 3. Guess next mask and const bit, recurse
    for (uint64_t mask = 0; mask < NUM; mask++) {
        // vector<word> Xnew[2];
        // split X into two sets according to the mask evaluation
        // split X into two halves in place (two-pointer method)
        int i = 0;
        int j = Xsize-1;
        // invariant: X[..i-1] has const 0, X[j+1 ...] has const 1
        while (i <= j) {
            assert(i < NUM);
            assert(j >= -1);
            // NOTE: LSB-based index at output (depth 0 = output LSB)
            word x = X[i];
            uint8_t ybit = (S[x] >> depth) & 1;
            uint8_t xbit = __builtin_popcount(x & mask) & 1;
            if (xbit ^ ybit) {
                // const = 1: move to X[j], decrease j
                if (i != j) {
                    swap(X[i], X[j]);
                }
                j--;
            }
            else {
                // const = 0: keep in X[i], decrease i
                i++;
            }
        }
        assert(0 <= i && i <= Xsize);
        assert(i == j + 1);
        #if X_SPLIT_TESTING == 1
            // test the splitting (~20% overhead cost?)
            for (size_t k = 0; k < Xsize; k++) {
                word x = X[k];
                uint8_t ybit = (S[x] >> depth) & 1;
                uint8_t xbit = __builtin_popcount(x & mask) & 1;
                assert((xbit ^ ybit) == (k >= i));
            }
        #endif
        // recurse into two cases
        // note: recursion calls are allowed to reorder their segments
        // (for splitting or e.g. sorting to display a solution)
        recurse(depth+1, X, i, stat);
        recurse(depth+1, X+i, Xsize-i, stat);
    }
    return;
}

static void anf_in_place(vector<word> & anf, int n) {
    for (int i = 0; i < n; i++) {
        // xor up
        for (uint64_t j = 0; j < (1 << n); j++) {
            uint64_t j2 = j ^ (1 << i);
            if (j < j2) {
                anf[j2] ^= anf[j];
            }
        }
    }
}

static bool sbox_is_quadratic() {
    bool is_quadratic = true;
    vector<word> anf(S, S+NUM);
    anf_in_place(anf, BITS);
    for(uint64_t e = 0; e < NUM; e++) {
        if (__builtin_popcount(e) > 2) {
            is_quadratic &= !anf[e];
        }
    }
    printf("S-box is quadratic? %d\n", is_quadratic);
    fflush(stdout);
    return is_quadratic;
}

static void compute_derivatives_quadratic() {
    set<uint64_t> der_set;
    for (uint64_t delta = 0; delta < NUM; delta++) {
        vector<word> anf(S, S+NUM);
        for (uint64_t x = 0; x < NUM; x++) {
            anf[x] = S[x] ^ S[x ^ delta];
        }
        anf_in_place(anf, BITS);

        // note: output bits indexed starting from LSB
        // input bits are same as masks (LSB=LSB, MSB=MSB)
        uint64_t mask1 = 0;
        uint64_t mask2 = 0;
        uint64_t const1 = anf[0] & 1;
        uint64_t const2 = (anf[0] & 2) >> 1;
        for (int i = 0; i < BITS; i++) {
            uint64_t coef = anf[1 << i];
            uint64_t bit1 = coef & 1;
            uint64_t bit2 = (coef & 2) >> 1;
            mask1 |= bit1 << i;
            mask2 |= bit2 << i;
        }

        uint64_t derivative = 0;
        // derivatives with constant part?
        // derivative |= const2;
        // derivative |= mask2 << 1;
        // derivative |= (const1 << (BITS + 1));
        // derivative |= (mask1 << (BITS + 2));

        // only the linear part matters now
        derivative |= mask2;
        derivative |= (mask1 << BITS);
        
        DERIVATIVES.push_back(derivative);
        der_set.insert(derivative);

        // testing
        for (uint64_t x = 0; x < NUM; x++) {
            uint64_t y = S[x] ^ S[x ^ delta];
            uint64_t val1 = (__builtin_popcount(x & mask1) & 1) ^ const1;
            uint64_t val2 = (__builtin_popcount(x & mask2) & 1) ^ const2;
            assert((y&1) == val1);
            assert( ((y&2) >> 1) == val2);
        }
    }
    printf("Computed %lu derivatives (%lu unique)\n", DERIVATIVES.size(), der_set.size());
    fflush(stdout);

    // keep unique ones
    DERIVATIVES.clear();
    for(auto der: der_set) {
        DERIVATIVES.push_back(der);
    }
    uint64_t n_der = DERIVATIVES.size();
    assert( (n_der & (n_der - 1)) == 0 ); // power of 2 - derivatives form a linear space
}

static void read_sbox() {
    printf("Input S-box (n m then space separated integers):\n");
    
    int BITS_OUT;
    assert(1 == scanf("%d", &BITS));
    assert(1 == scanf("%d", &BITS_OUT));
    NUM = 1 << BITS;
    printf("n = %d, m = %d, 2^n = %d\n", BITS, BITS_OUT, NUM);
    
    assert(BITS == BITS_OUT); // current assumption in the implementation

    #ifdef WORD_U8
        assert(2 <= BITS && BITS <= 8);
    #endif
    #ifdef WORD_U16
        assert(2 <= BITS && BITS < 16);
    #endif
    
    for(uint64_t x = 0; x < NUM; x++) {
        int y;
        assert(1 == scanf("%d", &y));
        S[x] = y;
        printf("%d ", y);
        assert(0 <= y && y < NUM);
    }
    printf("\n");
}

int main(int argc, char * argv[]) {
    int batch_index = 0;
    int batch_number = 1;

    if (argc == 2) {
        LB = atoi(argv[1]);
    }
    else if (argc == 4) {
        LB = atoi(argv[1]);
        batch_index = atoi(argv[2]);
        batch_number = atoi(argv[3]);
    }
    else {
        printf("Usage: %s <lower bound> [<batch-index> <batch-number>]  < sbox.txt\n", argv[0]);
        return -1;
    }
    
    assert(0 <= LB);
    assert(1 <= batch_number);
    assert(0 <= batch_index && batch_index < batch_number);
    
    read_sbox();

    int test = 0;
    assert(scanf("%d", &test) < 1); // EOF, ensure full file read
    
    assert(LB <= NUM);

    if (sbox_is_quadratic()) {
        compute_derivatives_quadratic();
    }
    printf("\n");
    
    printf("RUNNING WITH LB %d, BATCH index=%d (%d/%d)\n", LB, batch_index, batch_index + 1, batch_number);
    printf("Options: FULL_STAT %d, X_SPLIT_TESTING %d\n", FULL_STAT, X_SPLIT_TESTING);
    fflush(stdout);

    // do 256 prints
    uint64_t print_interval = NUM * NUM / 256;
    if (DERIVATIVES.size()) {
        print_interval /= DERIVATIVES.size();
    }
    if (!print_interval) {
        print_interval = 1;
    }
    uint64_t print_wait = 0;

    // we process the first two masks separately
    // to allow efficient parallelization
    vector<pair<uint64_t, uint64_t>> first_masks_list;
    for (uint64_t mask1 = 0; mask1 < NUM; mask1++) {
    for (uint64_t mask2 = 0; mask2 < NUM; mask2++) {
        first_masks_list.push_back({mask1, mask2});
    }}
    
    // randomize their order to mix hard/easy cases across threads
    mt19937 rand(uint64_t(12345)); // seed
    shuffle(first_masks_list.begin(), first_masks_list.end(), rand);

    STAT full_stat(BITS+1);
    size_t prefixes_checked = 0;

    const size_t n_itr = first_masks_list.size();
    #pragma omp parallel for
    for (uint64_t itr = 0; itr < n_itr; itr++) {
        if (itr % batch_number != batch_index) {
            continue;
        }
        uint64_t mask1 = first_masks_list[itr].first;
        uint64_t mask2 = first_masks_list[itr].second;

        STAT cur_stat(BITS+1); 
        
        // only linear derivatives
        // this derivative should be lexicographically smallest
        uint64_t full_prefix = (mask1 << BITS) | mask2;
        bool skip = false;
        for (auto der: DERIVATIVES) {
            if ((der ^ full_prefix) < full_prefix) {
                skip = true;
                cur_stat.prefix_skips++;
                break;
            }
        }
        if (!skip) {
            // printing
            #pragma omp critical
            {
                prefixes_checked++;
                if (print_wait) {
                    print_wait--;
                }
                else {
                    printf("checking masks itr=%8lu (%02lx %02lx) thread %d/%d\n",
                        itr, mask1, mask2, omp_get_thread_num(), omp_get_num_threads());
                    fflush(stdout);
                    print_wait = print_interval-1;
                }
            }
            
            // for each constant bit for the first two masks
            // recurse
            for(uint64_t const1 = 0; const1 < 2; const1++) {
            for(uint64_t const2 = 0; const2 < 2; const2++) {
                recurse_start(mask1, const1, mask2, const2, cur_stat);
            }}
        }

        #pragma omp critical
        full_stat.add(cur_stat);
    }
    printf("\n");

    full_stat.report();
    printf("main loop prefixes checked: %lu\n", prefixes_checked);
    assert(prefixes_checked + full_stat.prefix_skips == NUM * NUM);

    printf("Finished\n");
    fflush(stdout);
    return 0;
}