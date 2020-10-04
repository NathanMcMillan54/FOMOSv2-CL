use crate::commands::echo::echo::echo;
pub(crate) fn shutdown() {
    echo(b"\nShutting down... \n");
}
