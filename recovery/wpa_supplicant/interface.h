#ifndef FOMOSV2_CL_INTERFACE_H
#define FOMOSV2_CL_INTERFACE_H

#include <QtCore/QObject>
#include <QtCore/QByteArray>
#include <QtCore/QList>
#include <QtCore/QMap>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVariant>
#include <QtDBus/QtDBus>

/*
 * Proxy class for interface fi.w1.wpa_supplicant1.Interface
 */
class FiW1Wpa_supplicant1InterfaceInterface: public QDBusAbstractInterface
{
    Q_OBJECT
public:
    static inline const char *staticInterfaceName()
    { return "fi.w1.wpa_supplicant1.Interface"; }

public:
    FiW1Wpa_supplicant1InterfaceInterface(const QString &service, const QString &path, const QDBusConnection &connection, QObject *parent = 0);

    ~FiW1Wpa_supplicant1InterfaceInterface();

    Q_PROPERTY(uint ApScan READ apScan WRITE setApScan)
    inline uint apScan() const
    { return qvariant_cast< uint >(property("ApScan")); }
    inline void setApScan(uint value)
    { setProperty("ApScan", QVariant::fromValue(value)); }

    Q_PROPERTY(uint BSSExpireAge READ bSSExpireAge WRITE setBSSExpireAge)
    inline uint bSSExpireAge() const
    { return qvariant_cast< uint >(property("BSSExpireAge")); }
    inline void setBSSExpireAge(uint value)
    { setProperty("BSSExpireAge", QVariant::fromValue(value)); }

    Q_PROPERTY(uint BSSExpireCount READ bSSExpireCount WRITE setBSSExpireCount)
    inline uint bSSExpireCount() const
    { return qvariant_cast< uint >(property("BSSExpireCount")); }
    inline void setBSSExpireCount(uint value)
    { setProperty("BSSExpireCount", QVariant::fromValue(value)); }

    Q_PROPERTY(QList<QDBusObjectPath> BSSs READ bSSs)
    inline QList<QDBusObjectPath> bSSs() const
    { return qvariant_cast< QList<QDBusObjectPath> >(property("BSSs")); }

    Q_PROPERTY(QVariantMap Blobs READ blobs)
    inline QVariantMap blobs() const
    { return qvariant_cast< QVariantMap >(property("Blobs")); }

    Q_PROPERTY(QString BridgeIfname READ bridgeIfname)
    inline QString bridgeIfname() const
    { return qvariant_cast< QString >(property("BridgeIfname")); }

    Q_PROPERTY(QVariantMap Capabilities READ capabilities)
    inline QVariantMap capabilities() const
    { return qvariant_cast< QVariantMap >(property("Capabilities")); }

    Q_PROPERTY(QString Country READ country WRITE setCountry)
    inline QString country() const
    { return qvariant_cast< QString >(property("Country")); }
    inline void setCountry(const QString &value)
    { setProperty("Country", QVariant::fromValue(value)); }

    Q_PROPERTY(QString CurrentAuthMode READ currentAuthMode)
    inline QString currentAuthMode() const
    { return qvariant_cast< QString >(property("CurrentAuthMode")); }

    Q_PROPERTY(QDBusObjectPath CurrentBSS READ currentBSS)
    inline QDBusObjectPath currentBSS() const
    { return qvariant_cast< QDBusObjectPath >(property("CurrentBSS")); }

    Q_PROPERTY(QDBusObjectPath CurrentNetwork READ currentNetwork)
    inline QDBusObjectPath currentNetwork() const
    { return qvariant_cast< QDBusObjectPath >(property("CurrentNetwork")); }

    Q_PROPERTY(int DisconnectReason READ disconnectReason)
    inline int disconnectReason() const
    { return qvariant_cast< int >(property("DisconnectReason")); }

    Q_PROPERTY(QString Driver READ driver)
    inline QString driver() const
    { return qvariant_cast< QString >(property("Driver")); }

    Q_PROPERTY(bool FastReauth READ fastReauth WRITE setFastReauth)
    inline bool fastReauth() const
    { return qvariant_cast< bool >(property("FastReauth")); }
    inline void setFastReauth(bool value)
    { setProperty("FastReauth", QVariant::fromValue(value)); }

    Q_PROPERTY(QString Ifname READ ifname)
    inline QString ifname() const
    { return qvariant_cast< QString >(property("Ifname")); }

    Q_PROPERTY(QList<QDBusObjectPath> Networks READ networks)
    inline QList<QDBusObjectPath> networks() const
    { return qvariant_cast< QList<QDBusObjectPath> >(property("Networks")); }

    Q_PROPERTY(QString PKCS11EnginePath READ pKCS11EnginePath)
    inline QString pKCS11EnginePath() const
    { return qvariant_cast< QString >(property("PKCS11EnginePath")); }

    Q_PROPERTY(QString PKCS11ModulePath READ pKCS11ModulePath)
    inline QString pKCS11ModulePath() const
    { return qvariant_cast< QString >(property("PKCS11ModulePath")); }

    Q_PROPERTY(int ScanInterval READ scanInterval WRITE setScanInterval)
    inline int scanInterval() const
    { return qvariant_cast< int >(property("ScanInterval")); }
    inline void setScanInterval(int value)
    { setProperty("ScanInterval", QVariant::fromValue(value)); }

    Q_PROPERTY(bool Scanning READ scanning)
    inline bool scanning() const
    { return qvariant_cast< bool >(property("Scanning")); }

    Q_PROPERTY(QString State READ state)
    inline QString state() const
    { return qvariant_cast< QString >(property("State")); }

public Q_SLOTS: // METHODS
    inline QDBusPendingReply<> AddBlob(const QString &name, const QByteArray &data)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(name) << QVariant::fromValue(data);
        return asyncCallWithArgumentList(QLatin1String("AddBlob"), argumentList);
    }

    inline QDBusPendingReply<QDBusObjectPath> AddNetwork(const QVariantMap &args)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(args);
        return asyncCallWithArgumentList(QLatin1String("AddNetwork"), argumentList);
    }

    inline QDBusPendingReply<> Disconnect()
    {
        QList<QVariant> argumentList;
        return asyncCallWithArgumentList(QLatin1String("Disconnect"), argumentList);
    }

    inline QDBusPendingReply<> EAPLogoff()
    {
        QList<QVariant> argumentList;
        return asyncCallWithArgumentList(QLatin1String("EAPLogoff"), argumentList);
    }

    inline QDBusPendingReply<> EAPLogon()
    {
        QList<QVariant> argumentList;
        return asyncCallWithArgumentList(QLatin1String("EAPLogon"), argumentList);
    }

    inline QDBusPendingReply<> FlushBSS(uint age)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(age);
        return asyncCallWithArgumentList(QLatin1String("FlushBSS"), argumentList);
    }

    inline QDBusPendingReply<QByteArray> GetBlob(const QString &name)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(name);
        return asyncCallWithArgumentList(QLatin1String("GetBlob"), argumentList);
    }

    inline QDBusPendingReply<> NetworkReply(const QDBusObjectPath &path, const QString &field, const QString &value)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(path) << QVariant::fromValue(field) << QVariant::fromValue(value);
        return asyncCallWithArgumentList(QLatin1String("NetworkReply"), argumentList);
    }

    inline QDBusPendingReply<> Reassociate()
    {
        QList<QVariant> argumentList;
        return asyncCallWithArgumentList(QLatin1String("Reassociate"), argumentList);
    }

    inline QDBusPendingReply<> RemoveAllNetworks()
    {
        QList<QVariant> argumentList;
        return asyncCallWithArgumentList(QLatin1String("RemoveAllNetworks"), argumentList);
    }

    inline QDBusPendingReply<> RemoveBlob(const QString &name)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(name);
        return asyncCallWithArgumentList(QLatin1String("RemoveBlob"), argumentList);
    }

    inline QDBusPendingReply<> RemoveNetwork(const QDBusObjectPath &path)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(path);
        return asyncCallWithArgumentList(QLatin1String("RemoveNetwork"), argumentList);
    }

    inline QDBusPendingReply<> Scan(const QVariantMap &args)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(args);
        return asyncCallWithArgumentList(QLatin1String("Scan"), argumentList);
    }

    inline QDBusPendingReply<> SelectNetwork(const QDBusObjectPath &path)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(path);
        return asyncCallWithArgumentList(QLatin1String("SelectNetwork"), argumentList);
    }

    inline QDBusPendingReply<> SetPKCS11EngineAndModulePath(const QString &pkcs11_engine_path, const QString &pkcs11_module_path)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(pkcs11_engine_path) << QVariant::fromValue(pkcs11_module_path);
        return asyncCallWithArgumentList(QLatin1String("SetPKCS11EngineAndModulePath"), argumentList);
    }

    inline QDBusPendingReply<> SubscribeProbeReq()
    {
        QList<QVariant> argumentList;
        return asyncCallWithArgumentList(QLatin1String("SubscribeProbeReq"), argumentList);
    }

    inline QDBusPendingReply<> UnsubscribeProbeReq()
    {
        QList<QVariant> argumentList;
        return asyncCallWithArgumentList(QLatin1String("UnsubscribeProbeReq"), argumentList);
    }

    Q_SIGNALS: // SIGNALS
            void BSSAdded(const QDBusObjectPath &path, const QVariantMap &properties);
    void BSSRemoved(const QDBusObjectPath &path);
    void BlobAdded(const QString &name);
    void BlobRemoved(const QString &name);
    void Certification(const QVariantMap &certification);
    void EAP(const QString &status, const QString &parameter);
    void NetworkAdded(const QDBusObjectPath &path, const QVariantMap &properties);
    void NetworkRemoved(const QDBusObjectPath &path);
    void NetworkRequest(const QDBusObjectPath &path, const QString &field, const QString &text);
    void NetworkSelected(const QDBusObjectPath &path);
    void ProbeRequest(const QVariantMap &args);
    void PropertiesChanged(const QVariantMap &properties);
    void ScanDone(bool success);
    void StaAuthorized(const QString &name);
    void StaDeauthorized(const QString &name);
};


#endif //FOMOSV2_CL_INTERFACE_H
