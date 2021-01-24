#include <linux/reboot.h>
#include <stdlib.h>
#include <sys/reboot.h>
#include <unistd.h>

void shutdown(int s) {
    sleep(s);
    reboot(LINUX_REBOOT_CMD_POWER_OFF);
}