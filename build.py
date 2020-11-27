import os


def main():
    print("Compiling FOMOSv2-CL...")
    os.system('cargo build')
    os.system('gcc -static linux/main.c target/debug/libFOMOSv2_CL.a -o init')
    os.system('find init | cpio -o -H newc | gzip > rootfs.cpio.gz')
    print("FOMOSv2-CL image crated ")


if __name__ == '__main__':
    main()
