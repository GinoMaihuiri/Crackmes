#include <stdio.h>
#include <stdlib.h>
#include "../../common/ui.h"

int main(void) {
    print_banner(1, 2, "Serial numerico",
        "Solo un numero desbloquea el sistema.");

    char input[256];
    read_input("Serial     : ", input, sizeof(input));

    int val = atoi(input);
    if (val == 0x539) {
        print_ok("Serial valido!");
        return 0;
    }
    print_err("Serial invalido.");
    return 1;
}
