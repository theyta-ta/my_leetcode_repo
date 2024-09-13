#include <string.h>

char* longestCommonPrefix(char** strs, int strsSize) {
    int i = 0, j;
    char c, tmp;

    while (1) {
        c = strs[0][i];

        if (c == '\0')
            return strndup(strs[0], i);
        
        for (j = 1; j < strsSize; j++) {
            tmp = strs[j][i];
            if (tmp != c || tmp == '\0')
                return strndup(strs[0], i);
        }
        i++;
    }
}
