#ifndef FOMOSV2_CL_NETWORK_H
#define FOMOSV2_CL_NETWORK_H

#include <QtCore/QObject>
#include <QtCore/QByteArray>
#include <QtCore/QList>
#include <QtCore/QMap>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVariant>
#include <QtDBus/QtDBus>

/*
 * Proxy class for interface fi.w1.wpa_supplicant1.Network
 */
class FiW1Wpa_supplicant1NetworkInterface: public QDBusAbstractInterface
{
    Q_OBJECT
public:
    static inline const char *staticInterfaceName()
    { return "fi.w1.wpa_supplicant1.Network"; }

public:
    FiW1Wpa_supplicant1NetworkInterface(const QString &service, const QString &path, const QDBusConnection &connection, QObject *parent = 0);

    ~FiW1Wpa_supplicant1NetworkInterface();

    Q_PROPERTY(bool Enabled READ enabled WRITE setEnabled)
    inline bool enabled() const
    { return qvariant_cast< bool >(property("Enabled")); }
    inline void setEnabled(bool value)
    { setProperty("Enabled", QVariant::fromValue(value)); }

    Q_PROPERTY(QVariantMap Properties READ properties WRITE setProperties)
    inline QVariantMap properties() const
    { return qvariant_cast< QVariantMap >(property("Properties")); }
    inline void setProperties(const QVariantMap &value)
    { setProperty("Properties", QVariant::fromValue(value)); }

public Q_SLOTS: // METHODS
            Q_SIGNALS: // SIGNALS
    void PropertiesChanged();
};

#endif //FOMOSV2_CL_NETWORK_H
