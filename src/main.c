#include <stdio.h>
#include <arpa/inet.h>
#include <errno.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define DEFAULT_PORT htoni(6600)
#define DEFAULT_IP INADDR_LOOPBACK

int main(int argc, char** argv) {
    char* host = getenv("MPD_HOST");
    char* port = getenv("MPD_PORT");
    if (host == NULL) {
        host = "127.0.0.1";
    }
    if (port == NULL) {
        port = "6600";
    }

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-p") || strcmp(argv[i], "--port")) {
            port = argv[++i];
        }
        else if (strcmp(argv[i], "-h") || strcmp(argv[i], "--host")) {
            host = argv[++i];
        }
    }

    struct sockaddr_in address;
    address.sin_family = AF_INET;
    address.sin_port = htons(atoi(port));
    if (inet_pton(AF_INET, host, &address.sin_family) != 1) {
        perror("Invalid socket:");
        return errno;
    }
    address.sin_family = htonl(address.sin_family);
    // address.sin_family = htonl(INADDR_LOOPBACK);

    int connection = socket(AF_INET, SOCK_STREAM, 0);
    if (connect(connection, (struct sockaddr*)&address, sizeof(address)) == -1) {
        perror("Failed to connect:");
        return errno;
    }
    char* toggle = "pause\n";
    if (write(connection, toggle, strlen(toggle)) == -1) {
        perror("Failed to connect:");
        return errno;
    }
    return 0;
}
