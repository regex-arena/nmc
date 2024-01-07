#include <ctype.h>
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
    int len = datalen;
    while (strstr(data, "\nOK\n") == NULL) {
        int result = read(fd, data + offset, len - offset);
        if (result == -1) {
            perror("Failed to read");
            exit(errno);
        }
        if (offset + result >= len) {
            len *= 2;
            data = realloc(data, len);
            if (data == NULL) {
                perror("Memory allocation failed");
                exit(errno);
            }
        }
        offset += result;
    }
    return data;
}

int compare(const void* a, const void* b) {
    return (*(int*)b)-(*(int*)a);
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
            // Get discard state
            bwrite(connection, "status\n");
            char* buffer = malloc(100*sizeof(char));
            buffer = fullbread(connection, buffer, 100);
            char* val = strstr(buffer, "consume: ") + strlen("consume: ");
            int state = atoi(val);
            free(buffer);
            if (state) {
                bwrite(connection, "next\n");
                char* buffer = malloc(100*sizeof(char));
                bread(connection, buffer, 100);
                free(buffer);
            } else {
                bwrite(connection,
                       "command_list_begin\n"
                       "consume 1\n"
                       "next\n"
                       "consume 0\n"
                       "command_list_end\n"
                       );
                char* buffer = malloc(100*sizeof(char));
                bread(connection, buffer, 100);
                free(buffer);
            }
        } else if (!strcmp(argv[i], "status")) {
            bwrite(connection, "status\n");
            char* buffer = malloc(100*sizeof(char));
            buffer = fullbread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "playlist")) {
            bwrite(connection, "playlistinfo\n");
            char* buffer = malloc(100*sizeof(char));
            buffer = fullbread(connection, buffer, 100);

            char* token = strstr(buffer, "file: ");
            for (i = 1; token != NULL; i++) {
                // Workaround as adding to NULL creates segfault
                token += strlen("file: ");
                printf("%d: ", i);
                // Print to newline
                for (int j = 0; token[j] != '\n'; j++) {
                    putchar(token[j]);
                }
                putchar('\n');
                token = strstr(token, "file: ");
            }
            free(buffer);
        } else if (!strcmp(argv[i], "repeat")) {
            // Get repeat state
            bwrite(connection, "status\n");
            char* buffer = malloc(100*sizeof(char));
            buffer = fullbread(connection, buffer, 100);
            char* val = strstr(buffer, "repeat: ") + strlen("repeat: ");
            int state = atoi(val);
            free(buffer);
            // Toggle state
            if (state) {
                bwrite(connection, "repeat 0\n");
            } else {
                bwrite(connection, "repeat 1\n");
            }
            buffer = malloc(100*sizeof(char));
            bread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "random")) {
            // Get random state
            bwrite(connection, "status\n");
            char* buffer = malloc(100*sizeof(char));
            buffer = fullbread(connection, buffer, 100);
            char* val = strstr(buffer, "random: ") + strlen("random: ");
            int state = atoi(val);
            free(buffer);
            // Toggle state
            if (state) {
                bwrite(connection, "random 0\n");
            } else {
                bwrite(connection, "random 1\n");
            }
            buffer = malloc(100*sizeof(char));
            bread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "single")) {
            // Get single state
            bwrite(connection, "status\n");
            char* buffer = malloc(100*sizeof(char));
            buffer = fullbread(connection, buffer, 100);
            char* val = strstr(buffer, "single: ") + strlen("single: ");
            int state = atoi(val);
            free(buffer);
            // Toggle state
            if (state) {
                bwrite(connection, "single 0\n");
            } else {
                bwrite(connection, "single 1\n");
            }
            buffer = malloc(100*sizeof(char));
            bread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "consume")) {
            // Get consume state
            bwrite(connection, "status\n");
            char* buffer = malloc(100*sizeof(char));
            buffer = fullbread(connection, buffer, 100);
            char* val = strstr(buffer, "consume: ") + strlen("consume: ");
            int state = atoi(val);
            free(buffer);
            // Toggle state
            if (state) {
                bwrite(connection, "consume 0\n");
            } else {
                bwrite(connection, "consume 1\n");
            }
            buffer = malloc(100*sizeof(char));
            bread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "update")) {
            // Get update state
            bwrite(connection, "update\n");
            char* buffer = malloc(100*sizeof(char));
            bread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "volume")) {
            int ammount = atoi(argv[++i]);
            if (!isdigit(argv[i][0])) {
                bwrite(connection, "getvol\n");
                char* buffer = malloc(100*sizeof(char));
                buffer = fullbread(connection, buffer, 100);
                char* val = strstr(buffer, "volume: ") + strlen("volume: ");
                ammount += atoi(val);
                free(buffer);
            }
            // Clamp ammount between 0 and 100
            ammount = (ammount > 100) ? 100 : ammount;
            ammount = (ammount < 0) ? 0 : ammount;

            // Format strings
            char * buf = malloc(strlen("setvol 100\n") +  1);
            snprintf(buf, strlen("setvol 100\n") + 1, "setvol %d\n", ammount);

            bwrite(connection, buf);
            char* buffer = malloc(100*sizeof(char));
            bread(connection, buffer, 100);
            free(buffer);
        } else if (!strcmp(argv[i], "add")) {
            for (char* args = strtok(argv[++i], ","); args != NULL; args = strtok(NULL, ",")) {
                int length = strlen("searchadd \"(file == \\\"\\\")\"\n")
                    + strlen(args)
                    + 1;
                char* command = malloc(length);
                snprintf(command, length, "searchadd \"(file == \\\"%s\\\")\"\n", args);
                bwrite(connection, command);
                bread(connection, command, length);
            }
        } else if (!strcmp(argv[i], "remove")) {
            char* args = strtok(argv[++i], ",");
            int len = 10;
            int* index = malloc(len*sizeof(int));
            int j;
            for (j = 0; args != NULL; j++) {
                if (j > len) {
                    len *= 2;
                    index = realloc(index, len*sizeof(int));
                }
                index[j] = atoi(args);
                args = strtok(NULL, ",");
            };
            qsort(index, j, sizeof(int), &compare);
            for (int i = 0; i < j; i++) {
                int length = strlen("delete 100000\n")+1;
                char* buf = malloc(length);
                // Short cast used to not overflow buffer
                snprintf(buf, length, "delete %d\n", (short)index[i]-1);
                bwrite(connection, buf);
                bread(connection, buf, length);
                free(buf);
            }
        } else if (!strcmp(argv[i], "help")) {
            printf("Runs all given commands, from left to right\n\n"
                   "Arguments:\n"
                   "-p/--port - changes mpd port from default 6600\n"
                   "-h/--host - changes mpd host from default 127.0.0.1\n\n"
                   "Commands\n"
                   "help      - Prints help screen\n"
                   "toggle    - Toggles mpd playback\n"
                   "discard   - removes curently playing song\n"
                   "status    - Same as no arguments: Prints mpd status screen\n"
                   "playlist  - outputs mpd playlist with index numbers\n"
                   "repeat    - toggles mpd consume\n"
                   "random    - toggles mpd random\n"
                   "single    - toggles mpd single\n"
                   "consume   - toggles mpd consume\n"
                   "update    - updates mpd database\n"
                   "volume    - changes mpd volume\n"
                   "add       - adds given files from mpd music directiory\n"
                   "            seperate list of files with commas\n"
                   "remove    - removes items at given indecies\n"
                   "            seperate list of indecies with commas\n");
        }
    }
    return 0;
}
