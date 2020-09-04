#ifndef FOMOSV2_CL_ATAGS_H
#define FOMOSV2_CL_ATAGS_H

void convert_to_tag_list(struct tag *tags);

#ifdef CONFIG_ATAGS
const struct machine_desc *setup_machine_tags(phys_addr_t __atags_pointer,
	unsigned int machine_nr);
#else
static inline const struct machine_desc * __init __noreturn
setup_machine_tags(phys_addr_t __atags_pointer, unsigned int machine_nr)
{
    early_print("no ATAGS support: can't continue\n");
    while (true);
    unreachable();
}

#endif //FOMOSV2_CL_ATAGS_H
