import os
import shutil

init = 'init'
rootfs = 'rootfs.cpio.gz'
target = 'target'


def dosent_exist(name):
    print(name, "doesn't exist.")


def delete_file(file):
    if os.path.exists(file):
        print("Deleting", file)
        os.remove(file)
    else:
        dosent_exist(file)


def delete_directory(directory):
    if os.path.exists(directory):
        print("Deleting", directory)
        shutil.rmtree(directory)
    else:
        dosent_exist(directory)


def main():
    print("Deleting compiled files")
    delete_directory(target)
    delete_file(init)
    delete_file(rootfs)
    print("Finished deleting files")


if __name__ == '__main__':
    main()
