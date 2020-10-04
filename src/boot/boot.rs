use crate::commands::echo::echo::echo;
use crate::setup::setup;

pub(crate) fn boot() {
    echo(b"Booting... Loading FOMOS... \n");
    setup();
}
