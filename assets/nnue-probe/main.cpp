#include <iostream>
#include "src/nnue.h"  

const char* NNUE_FILE_PATH = "./nn-04cf2b4ed1da.nnue";

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " <fen_string>" << std::endl;
        return 1;
    }

    const char* fen = argv[1];

    std::cout << "Loading NNUE file: " << NNUE_FILE_PATH << std::endl;
    nnue_init(NNUE_FILE_PATH);

    int score = nnue_evaluate_fen(fen);
    std::cout << "Eval score: " << score << std::endl;

    return 0;
}
