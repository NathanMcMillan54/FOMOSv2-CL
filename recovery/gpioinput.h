#ifndef FOMOSV2_CL_GPIOINPUT_H
#define FOMOSV2_CL_GPIOINPUT_H

#include <QByteArray>

class GpioInput
{
public:
    GpioInput(int number);
    virtual ~GpioInput();
    int value();

protected:
    int _number;
    QByteArray _numberStr;
};

#endif //FOMOSV2_CL_GPIOINPUT_H
