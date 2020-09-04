#ifndef FOMOSV2_CL_RIGHTBUTTONFILTER_H
#define FOMOSV2_CL_RIGHTBUTTONFILTER_H

#include <QObject>

class RightButtonFilter : public QObject
{
    Q_OBJECT
public:
    explicit RightButtonFilter(QObject *parent = 0);
    virtual bool eventFilter(QObject *, QEvent *event);

    signals:

public slots:

};

#endif //FOMOSV2_CL_RIGHTBUTTONFILTER_H
