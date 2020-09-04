#ifndef FOMOSV2_CL_JSON_H
#define FOMOSV2_CL_JSON_H

#include <QVariant>

class Json
{
public:
    static QVariant parse(const QByteArray &json);
    static QByteArray serialize(const QVariant &json);
    static QVariant loadFromFile(const QString &filename);
    static void saveToFile(const QString &filename, const QVariant &json);
};

#endif //FOMOSV2_CL_JSON_H
