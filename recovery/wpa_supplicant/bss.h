#ifndef FOMOSV2_CL_BSS_H
#define FOMOSV2_CL_BSS_H

#include <QtCore/QObject>
#include <QtCore/QByteArray>
#include <QtCore/QList>
#include <QtCore/QMap>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVariant>
#include <QtDBus/QtDBus>

/*
 * Proxy class for interface fi.w1.wpa_supplicant1.BSS
 */
class FiW1Wpa_supplicant1BSSInterface: public QDBusAbstractInterface
{
    Q_OBJECT
public:
    static inline const char *staticInterfaceName()
    { return "fi.w1.wpa_supplicant1.BSS"; }

public:
    FiW1Wpa_supplicant1BSSInterface(const QString &service, const QString &path, const QDBusConnection &connection, QObject *parent = 0);

    ~FiW1Wpa_supplicant1BSSInterface();

    Q_PROPERTY(QByteArray BSSID READ bSSID)
    inline QByteArray bSSID() const
    { return qvariant_cast< QByteArray >(property("BSSID")); }

    Q_PROPERTY(ushort Frequency READ frequency)
    inline ushort frequency() const
    { return qvariant_cast< ushort >(property("Frequency")); }

    Q_PROPERTY(QByteArray IEs READ iEs)
    inline QByteArray iEs() const
    { return qvariant_cast< QByteArray >(property("IEs")); }

    Q_PROPERTY(QString Mode READ mode)
    inline QString mode() const
    { return qvariant_cast< QString >(property("Mode")); }

    Q_PROPERTY(bool Privacy READ privacy)
    inline bool privacy() const
    { return qvariant_cast< bool >(property("Privacy")); }

    Q_PROPERTY(QVariantMap RSN READ rSN)
    inline QVariantMap rSN() const
    { return qvariant_cast< QVariantMap >(property("RSN")); }

    Q_PROPERTY(QStringList Rates READ rates)
    inline QStringList rates() const
    { return qvariant_cast< QStringList >(property("Rates")); }

    Q_PROPERTY(QByteArray SSID READ sSID)
    inline QByteArray sSID() const
    { return qvariant_cast< QByteArray >(property("SSID")); }

    Q_PROPERTY(short Signal READ signal)
    inline short signal() const
    { return qvariant_cast< short >(property("Signal")); }

    Q_PROPERTY(QVariantMap WPA READ wPA)
    inline QVariantMap wPA() const
    { return qvariant_cast< QVariantMap >(property("WPA")); }

    Q_PROPERTY(QVariantMap WPS READ wPS)
    inline QVariantMap wPS() const
    { return qvariant_cast< QVariantMap >(property("WPS")); }

public Q_SLOTS: // METHODS
            Q_SIGNALS: // SIGNALS
    void PropertiesChanged(const QVariantMap &properties);
};

#endif //FOMOSV2_CL_BSS_H
