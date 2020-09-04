#ifndef FOMOSV2_CL_KEYDETECTION_H
#define FOMOSV2_CL_KEYDETECTION_H

class KeyDetection
{
public:
    static bool isF10pressed();
    static bool waitForKeyboard();
protected:
    static int openKeyboard();
    static bool _isF10pressed(int fd);
};

#endif //FOMOSV2_CL_KEYDETECTION_H
