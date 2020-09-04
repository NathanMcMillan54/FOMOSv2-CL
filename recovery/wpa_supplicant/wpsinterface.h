#ifndef FOMOSV2_CL_WPSINTERFACE_H
#define FOMOSV2_CL_WPSINTERFACE_H

#include <QtCore/QObject>
#include <QtCore/QByteArray>
#include <QtCore/QList>
#include <QtCore/QMap>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVariant>
#include <QtDBus/QtDBus>

/*
 * Proxy class for interface fi.w1.wpa_supplicant1.Interface.WPS
 */
class FiW1Wpa_supplicant1InterfaceWPSInterface: public QDBusAbstractInterface
{
    Q_OBJECT
public:
    static inline const char *staticInterfaceName()
    { return "fi.w1.wpa_supplicant1.Interface.WPS"; }

public:
    FiW1Wpa_supplicant1InterfaceWPSInterface(const QString &service, const QString &path, const QDBusConnection &connection, QObject *parent = 0);

    ~FiW1Wpa_supplicant1InterfaceWPSInterface();

    Q_PROPERTY(bool ProcessCredentials READ processCredentials WRITE setProcessCredentials)
    inline bool processCredentials() const
    { return qvariant_cast< bool >(property("ProcessCredentials")); }
    inline void setProcessCredentials(bool value)
    { setProperty("ProcessCredentials", QVariant::fromValue(value)); }

public Q_SLOTS: // METHODS
    inline QDBusPendingReply<QVariantMap> Start(const QVariantMap &args)
    {
        QList<QVariant> argumentList;
        argumentList << QVariant::fromValue(args);
        return asyncCallWithArgumentList(QLatin1String("Start"), argumentList);
    }

    Q_SIGNALS: // SIGNALS
            void Credentials(const QVariantMap &credentials);
    void Event(const QString &name, const QVariantMap &args);
    void PropertiesChanged(const QVariantMap &properties);
};

#endif //FOMOSV2_CL_WPSINTERFACE_H
