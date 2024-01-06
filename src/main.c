#include <stdio.h>
#include <arpa/inet.h>
#include <errno.h>
#include <netinet/in.h>
#include <netinet/ip.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <sys/fcntl.h>
#include <stdlib.h>

#define DEFAULT_PORT htoni(6600)
#define DEFAULT_IP INADDR_LOOPBACK

void bwrite(int fd, const char* data) {
    if (write(fd, data, strlen(data)) == -1) {
        perror("Failed to write");
        exit(errno);
    }
}

void bread(int fd, char* data, int datalen) {
    if (recv(fd, data, datalen, 0) == -1) {
        perror("Failed to read");
        exit(errno);
    }
}

char* fullbread(int fd, char* data, int datalen) {
    int offset = 0;
    //for (;;) {
        int result = read(fd, data + offset, datalen);
        printf("%s\n", data);
        // if (result == -1) {
        //     perror("Failed to read");
        //     exit(errno);
        // } else if (result != datalen) {
        //     int result = read(fd, data + result, datalen);
        //     break;
        // } else {
        //     data = realloc(data, offset + datalen);
        //     if (data == NULL) {
        //         perror("Memory allocation failed");
        //         exit(errno);
        //     }
        //     offset += datalen;
        // }
    //}
    return data;
}

int main(int argc, char** argv) {
    char* host = getenv("MPD_HOST");
    char* port = getenv("MPD_PORT");
    if (host == NULL) {
        host = "127.0.0.1";
    }
    if (port == NULL) {
        port = "6600";
    }

    // Itterates over given arguments looking for host or port
    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "-p") || !strcmp(argv[i], "--port")) {
            port = argv[++i];
        }
        else if (!strcmp(argv[i], "-h") || !strcmp(argv[i], "--host")) {
            host = argv[++i];
        }
    }

    struct sockaddr_in address;
    address.sin_family = AF_INET;
    // Converts port string to (short) integer in network bite order
    address.sin_port = htons(atoi(port));
    // Converts ip string into appropriate network structure
    if (inet_pton(AF_INET, host, &address.sin_addr.s_addr) != 1) {
        perror("Invalid socket");
        return errno;
    }

    // Creates connection socket
    int connection = socket(AF_INET, SOCK_STREAM, 0);
    if (connection < 0) {
        perror("Unable to create socket");
        return errno;
    }

    // Connets socket to given port and address
    if (connect(connection, (struct sockaddr*)&address, sizeof(address)) == -1) {
        perror("Failed to connect");
        return errno;
    }


    // Itterates over given arguments looking for valid commands
    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "toggle")) {
            bwrite(connection, "pause\n");
            // Read reqired after every write
            char* buffer = malloc(101*sizeof(char));
            bread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "discard")) {
        } else if (!strcmp(argv[i], "status")) {
        } else if (!strcmp(argv[i], "playlist")) {
        } else if (!strcmp(argv[i], "repeat")) {
        } else if (!strcmp(argv[i], "random")) {
        } else if (!strcmp(argv[i], "single")) {
        } else if (!strcmp(argv[i], "consume")) {
            bwrite(connection, "status\n");
            char* buffer = malloc(101*sizeof(char));
            buffer = fullbread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "update")) {
            bwrite(connection, "update\n");
            char* buffer = malloc(1001*sizeof(char));
            bread(connection, buffer, 1000);
            free(buffer);
        } else if (!strcmp(argv[i], "volume")) {
            char *args = argv[++i];
        } else if (!strcmp(argv[i], "add")) {
            char *args = argv[++i];
        } else if (!strcmp(argv[i], "remove")) {
            char *args = argv[++i];
        } else if (!strcmp(argv[i], "help")) {
            printf("Runs all given commands, from left to right\n\n");
            printf("Arguments:\n");
            printf("-p/--port - changes mpd port from default 6600\n");
            printf("-h/--host - changes mpd host from default 127.0.0.1\n\n");
            printf("Commands\n");
            printf("help      - Prints help screen\n");
            printf("toggle    - Toggles mpd playback\n");
            printf("discard   - removes curently playing song\n");
            printf("status    - Same as no arguments: Prints mpd status screen\n");
            printf("playlist  - outputs mpd playlist with index numbers\n");
            printf("repeat    - toggles mpd consume\n");
            printf("random    - toggles mpd random\n");
            printf("single    - toggles mpd single\n");
            printf("consume   - toggles mpd consume\n");
            printf("update    - updates mpd database\n");
            printf("volume    - changes mpd volume\n");
            printf("add       - adds given files from mpd music directiory\n");
            printf("            seperate list of files with commas\n");
            printf("remove    - removes items at given indecies\n");
            printf("            seperate list of indecies with commas\n");
        }
    }
    return 0;
}
