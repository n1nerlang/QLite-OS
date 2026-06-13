#include <stddef.h>
#include <stdint.h>

void *qlite_memset(void *dest, int value, size_t count) {
    uint8_t *bytes = (uint8_t *)dest;
    for (size_t index = 0; index < count; ++index) {
        bytes[index] = (uint8_t)value;
    }
    return dest;
}

void *qlite_memcpy(void *dest, const void *src, size_t count) {
    uint8_t *out = (uint8_t *)dest;
    const uint8_t *in = (const uint8_t *)src;
    for (size_t index = 0; index < count; ++index) {
        out[index] = in[index];
    }
    return dest;
}

size_t qlite_strlen(const char *text) {
    size_t length = 0;
    while (text[length] != '\0') {
        ++length;
    }
    return length;
}

const char *qlite_runtime_banner(void) {
    return "QLite runtime ready";
}
