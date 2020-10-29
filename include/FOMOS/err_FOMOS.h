#ifndef FOMOSV2_CL_ERR_FOMOS_H
#define FOMOSV2_CL_ERR_FOMOS_H

void err_loop(int loop_count) {
	int loop;
	for (;;) {
		loop++;
		if (loop == loop_count) {
			break;
		}
	}
}

#endif //FOMOSV2_CL_ERR_FOMOS_H
