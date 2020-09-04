#include "interface.h"

/*
 * Implementation of interface class FiW1Wpa_supplicant1InterfaceInterface
 */

FiW1Wpa_supplicant1InterfaceInterface::FiW1Wpa_supplicant1InterfaceInterface(const QString &service, const QString &path, const QDBusConnection &connection, QObject *parent)
        : QDBusAbstractInterface(service, path, staticInterfaceName(), connection, parent)
{
}

FiW1Wpa_supplicant1InterfaceInterface::~FiW1Wpa_supplicant1InterfaceInterface()
{
}