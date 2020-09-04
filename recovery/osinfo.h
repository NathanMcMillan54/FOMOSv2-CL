#ifndef FOMOSV2_CL_OSINFO_H
#define FOMOSV2_CL_OSINFO_H

#include <QObject>
#include <QList>
#include <QStringList>

class PartitionInfo;

class OsInfo : public QObject
{
    Q_OBJECT
public:
    /* Constructor parses the json files in <folder>, and stores information in local variables */
    explicit OsInfo(const QString &folder, const QString &flavour = "", QObject *parent = 0);

    inline QString folder()
    {
        return _folder;
    }

    inline QString flavour()
    {
        return _flavour;
    }

    inline QString name()
    {
        return _name;
    }

    inline QString description()
    {
        return _description;
    }

    inline QString version()
    {
        return _version;
    }

    inline QString releaseDate()
    {
        return _releaseDate;
    }

    inline bool bootable()
    {
        return _bootable;
    }

    inline QList<PartitionInfo *> *partitions()
    {
        return &_partitions;
    }

    inline int riscosOffset()
    {
        return _riscosOffset;
    }

    inline QStringList supportedModels()
    {
        return _supportedModels;
    }

protected:
    QString _folder, _flavour, _name, _description, _version, _releaseDate;
    bool _bootable;
    QList<PartitionInfo *> _partitions;
    QStringList _supportedModels;
    int _riscosOffset;

};

#endif //FOMOSV2_CL_OSINFO_H
