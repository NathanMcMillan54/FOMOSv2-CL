#include "longpresshandler.h"
#include <QMouseEvent>
#include <QApplication>

LongPressHandler::LongPressHandler(QObject *parent) :
        QObject(parent), _holdInterval(250), _maxPosDifference(25)
{
}

bool LongPressHandler::eventFilter(QObject *obj, QEvent *event)
{
    if (event->type() == QEvent::MouseButtonPress)
    {
        QMouseEvent *mouseEvent = static_cast<QMouseEvent*>(event);
        _holdPoint = mouseEvent->globalPos();
        _holdTime.start();
    }
    else if (event->type() == QEvent::MouseButtonRelease)
    {
        QMouseEvent *mouseEvent = static_cast<QMouseEvent*>(event);
        QPoint posDiff = mouseEvent->globalPos() - _holdPoint;
        if (_holdTime.elapsed() > _holdInterval
            && posDiff.manhattanLength() <= _maxPosDifference)
        {
            QMouseEvent dblclk(QEvent::MouseButtonDblClick, mouseEvent->pos(), mouseEvent->button(), mouseEvent->buttons(), Qt::NoModifier);
            QApplication::sendEvent(obj, &dblclk);
        }
    }

    return false;
}
