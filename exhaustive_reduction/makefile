all: ReductionExhaustive8 ReductionExhaustive16

CXX=g++
# CXX=clang++

ReductionExhaustive8: ReductionExhaustive.cpp
	$(CXX) -fopenmp -std=c++2a -march=native -DWORD_U8 -O3 ReductionExhaustive.cpp -o ReductionExhaustive8

ReductionExhaustive16: ReductionExhaustive.cpp
	$(CXX) -fopenmp -std=c++2a -march=native -DWORD_U16 -O3 ReductionExhaustive.cpp -o ReductionExhaustive16

clean:
	rm -f ReductionExhaustive8 ReductionExhaustive16