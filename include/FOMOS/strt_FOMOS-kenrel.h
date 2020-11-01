#ifndef FOMOSV2_CL_STRT_FOMOS_KENREL_H
#define FOMOSV2_CL_STRT_FOMOS_KENREL_H

#include <linux/module.h>
#include <linux/kernel.h>

#include "err_FOMOS.h"
#include "../../src/fomos.h"

int power = 1;

void strt_fomos() {
  if (power == 1) {
    init();
  } else if (power == 0) {
    shutdown_fomos();
  } else {
    err_loop(999999);
    power = 1;
    init();
  }
}

#endif //FOMOSV2_CL_STRT_FOMOS_KENREL_H
