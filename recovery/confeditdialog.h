#ifndef FOMOSV2_CL_CONFEDITDIALOG_H
#define FOMOSV2_CL_CONFEDITDIALOG_H

#include <QDialog>
#include <QList>

namespace Ui {
    class ConfEditDialog;
}

class ConfEditDialogTab;

class ConfEditDialog : public QDialog
{
    Q_OBJECT

public:
    explicit ConfEditDialog(const QString &partition, QWidget *parent = 0);
    ~ConfEditDialog();

public slots:
    virtual void accept();

private:
    Ui::ConfEditDialog *ui;
    QList<ConfEditDialogTab *> _tabs;
};

#endif //FOMOSV2_CL_CONFEDITDIALOG_H
