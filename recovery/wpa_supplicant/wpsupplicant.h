#ifndef FOMOSV2_CL_WPSUPPLICANT_H
#define FOMOSV2_CL_WPSUPPLICANT_H

#include <QtCore/QObject>
#include <QtCore/QByteArray>
#include <QtCore/QList>
#include <QtCore/QMap>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVariant>
#include <QtDBus/QtDBus>

/*
 * Proxy class for interface fi.w1.wpa_supplicant1
 */
class FiW1Wpa_supplicant1Interface: public QDBusAbstractInterface
{
    Q_OBJECT
public:
    static inline const char *staticInterfaceName()
    { return "fi.w1.wpa_supplicant1"; }

public:
    FiW1Wpa_supplicant1Interface(const QString &service, const QString &path, const QDBusConnection &connection, QObject *parent = 0);

    ~FiW1Wpa_supplicant1Interface();

    Q_PROPERTY(QStringList Capabilities READ capabilities)
    inline QStringList capabilities() const
    { return qvariant_cast< QStringList >(property("Capabilities")); }

    Q_PROPERTY(QString DebugLevel READ debugLevel WRITE setDebugLevel)
    inline QString debugLevel() const
    { return qvariant_cast< QString >(property("DebugLevel")); }
    inline void setDebugLevel(const QString &value)
    { setProperty("DebugLevel", QVariant::fromValue(value)); }

    Q_PROPERTY(bool DebugShowKeys READ debugShowKeys WRITE setDebugShowKeys)
    inline bool debugShowKeys() const
    { return qvariant_cast< bool >(property("DebugShowKeys")); }
    inline void setDebugShowKeys(bool value)
    { setProperty("DebugShowKeys", QVariant::fromValue(value)); }

    Q_PROPERTY(bool DebugTimestamp READ debugTimestamp WRITE setDebugTimestamp)
    inline bool debugTimestamp() const
    { return qvariant_cast< bool >(property("DebugTimestamp")); }
    inline void setDebugTimestamp(bool value)
    { setProperty("DebugTimestamp", QVariant::fromValue(value)); }

    Q_PROPERTY(QStringList EapMethods READ eapMethods)
    inline QStringList eapMethods() const
    { return qvariant_cast< QStringList >(property("EapMethods")); }

    Q_PROPERTY(QList<QDBusObjectPath> Interfaces READ interfaces)
    inline QList<QDBusObjectPath> interfaces() const
    { return qvariant_cast< QList<QDBusObjectPath> >(property("Interfaces")); }

public Q_SLOTS: // METHODS
    inline QDBusPendingReply<QDBusObjectPath> CreateInterface(const QVariantMap &args)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(args);
        return asyncCallWithArgumentList(QLatin1String("CreateInterface"), argumentList);
    }

    inline QDBusPendingReply<QDBusObjectPath> GetInterface(const QString &ifname)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(ifname);
        return asyncCallWithArgumentList(QLatin1String("GetInterface"), argumentList);
    }

    inline QDBusPendingReply<> RemoveInterface(const QDBusObjectPath &path)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(path);
        return asyncCallWithArgumentList(QLatin1String("RemoveInterface"), argumentList);
    }

    Q_SIGNALS: // SIGNALS
            void InterfaceAdded(const QDBusObjectPath &path, const QVariantMap &properties);
    void InterfaceRemoved(const QDBusObjectPath &path);
    void PropertiesChanged(const QVariantMap &properties);
};


#endif //FOMOSV2_CL_WPSUPPLICANT_H
