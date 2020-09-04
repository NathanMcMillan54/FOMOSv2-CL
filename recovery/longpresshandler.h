#ifndef FOMOSV2_CL_LONGPRESSINFO_H
#define FOMOSV2_CL_LONGPRESSINFO_H

#include <QObject>
#include <QTime>
#include <QPoint>

class LongPressHandler : public QObject
{
    Q_OBJECT
public:
    explicit LongPressHandler(QObject *parent = 0);
    virtual bool eventFilter(QObject *obj, QEvent *event);

protected:
    QTime _holdTime;
    QPoint _holdPoint;
    int _holdInterval;
    int _maxPosDifference;
};

#endif //FOMOSV2_CL_LONGPRESSINFO_H
