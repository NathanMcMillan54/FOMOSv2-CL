use crate::commands::echo::echo::echo;
pub(crate) fn shutdown() {
    echo(b"Shutting down... \n");
}
