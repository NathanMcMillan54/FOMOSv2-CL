#ifndef FOMOSV2_CL_MBR_H
#define FOMOSV2_CL_MBR_H

struct mbr_partition_entry {
    unsigned char bootable;
    char begin_hsc[3];
    unsigned char id;
    char end_hsc[3];
    unsigned int starting_sector;
    unsigned int nr_of_sectors;
} __attribute__ ((packed));

struct mbr_table {
    char bootcode[440];
    unsigned char diskid[4];
    unsigned char flags[2];
    mbr_partition_entry part[4];
    unsigned char signature[2];
} __attribute__ ((packed));

#endif //FOMOSV2_CL_MBR_H
