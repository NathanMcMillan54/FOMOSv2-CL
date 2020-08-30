This directory is just for FOMOS's own 'libraries'. Instead of using ```stdio.h``` you'll be using ```output/print.h``` 
#

Imagine this: you try writing hello world
```c
#include <stdio.h>

int main() {
    print
```

But... oh no! you broke your finger trying to write ```printf```! Whatever shall you do?

Just use:
```c
#include "include/output/print.h"

int main() {
    print("hello world");
    return 0;
}
```
