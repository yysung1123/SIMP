#define assert(x) \
    do { \
        if (!(x)) return 1; \
    } while (0)

extern void __start(void);

int main() {
    // addu
    {
        int x = 47, y = -5, z = 42;
        assert((x + y == z));
    }

    // addiu
    {
        int x = 560, y = 563;
        assert(x + 3 == y);
    }

    // negu
    {
        int x = -563, y = 563;
        assert(-x == y);
    }

    // clo
    {
        int x = 0xffff0000, y = 16, z;
        asm volatile("clo %0,%1" : "=r"(z) : "r"(x):);
        assert(z == y);
    }

    // clz
    {
        int x = 0x0000ffff, y = 16;
        assert(__builtin_clz(x) == y);
    }

    // la
    {
        void (*x)(void);
        asm volatile("la %0, __start" : "=r"(x) ::);
        assert(x == __start);
    }

    // li
    {
        int x, y = 563;
        asm volatile("li %0, 563" : "=r"(x) ::);
        assert(x == y);
    }

    // lui
    {
        int x, y = 0xffff0000;
        asm volatile("lui %0, 65535" : "=r"(x) ::);
        assert(x == y);
    }

    // move
    {
        int x, y = 563;
        asm volatile("move %0, %1" : "=r"(x) : "r"(y) :);
        assert(x == y);
    }

    // subu
    {
        int x = 47, y = 5, z = 42;
        assert((x - y == z));
    }

    // sll
    {
        int x = 1 << 10, y = 1 << 12;
        assert((x << 2) == y);
    }

    // sllv
    {
        int x = 1 << 10, y = 2, z = 1 << 12;
        assert((x << y) == z);
    }

    // srl
    {
        int x = 1 << 10, y = 1 << 8;
        assert((x >> 2) == y);
    }

    // srlv
    {
        unsigned int x = 1 << 31, y = 2, z = 1 << 29;
        assert((x >> y) == z);
    }

    // sra
    {
        int x = -7, y = -1;
        assert((x >> 3) == y);
    }

    // srav
    {
        int x = -7, y = 3, z = -1;
        assert((x >> y) == z);
    }

    // and
    {
        int x = 0x11111111, y = 0x10101010;
        assert((x & y) == y);
    }

    // andi
    {
        int x = 0x1111, y = 0x1010;
        assert((x & 0x1010) == y);
    }

    // nor
    {
        int x = 0xf0f0f0f0, y = 0x0f0f0f0f, z;
        asm volatile("nor %0, %1, %2" : "=r"(z) : "r"(x), "r"(y) :);
        assert(z == 0);
    }

    // not
    {
        int x = 0xf0f0f0f0, y = 0x0f0f0f0f, z;
        asm volatile("not %0, %1" : "=r"(z) : "r"(x) :);
        assert(z == y);
    }

    // or
    {
        int x = 0x10101010, y = 0x01010101, z = 0x11111111;
        assert((x | y) == z);
    }

    // ori
    {
        int x = 0x1010, y = 0x1111;
        assert((x | 0x0101) == y);
    }

    // xor
    {
        int x = 0x11111111, y = 0x10101010, z = 0x01010101;
        assert((x ^ y) == z);
    }

    // xori
    {
        int x = 0x1111, y = 0x0101;
        assert((x ^ 0x1010) == y);
    }

    // slt
    {
        int x, y, z;
        x = -1;
        y = 1;
        asm volatile("slt %0,%1,%2" : "=r"(z) : "r"(x), "r"(y):);
        assert(z == 1);

        x = 1;
        y = -1;
        asm volatile("slt %0,%1,%2" : "=r"(z) : "r"(x), "r"(y):);
        assert(z == 0);
    }

    // slti
    {
        int x, y;
        x = -1;
        asm volatile("slt %0,%1,1" : "=r"(y) : "r"(x):);
        assert(y == 1);

        x = 1;
        asm volatile("slt %0,%1,-1" : "=r"(y) : "r"(x):);
        assert(y == 0);
    }

    // sltu
    {
        unsigned x, y;
        int z;
        x = 1;
        y = 65535;
        asm volatile("slt %0,%1,%2" : "=r"(z) : "r"(x), "r"(y):);
        assert(z == 1);
        x = 65535;
        y = 1;
        asm volatile("slt %0,%1,%2" : "=r"(z) : "r"(x), "r"(y):);
        assert(z == 0);
    }

    // sltu
    {
        unsigned x;
        int y;
        x = 1;
        asm volatile("slt %0,%1,65535" : "=r"(y) : "r"(x):);
        assert(y == 1);
        x = 65535;
        asm volatile("slt %0,%1,1" : "=r"(y) : "r"(x):);
        assert(y == 0);
    }

    // div
    {
        int x = -65, y = 8, z, w;
        asm volatile("div %0,%1" :: "r"(x), "r"(y):);
        asm volatile("mflo %0" : "=r"(z) ::);
        asm volatile("mfhi %0" : "=r"(w) ::);
        assert(z == -8);
        assert(w == -1);
    }

    // divu
    {
        unsigned int x = 65, y = 8, z, w;
        asm volatile("divu %0,%1" :: "r"(x), "r"(y):);
        asm volatile("mflo %0" : "=r"(z) ::);
        asm volatile("mfhi %0" : "=r"(w) ::);
        assert(z == 8);
        assert(w == 1);
    }

    // madd
    {
        asm volatile("mthi $0" :::);
        asm volatile("mtlo $0" :::);
        int x = 1234567, y = -1234567, z, w;
        asm volatile("madd %0,%1" :: "r"(x), "r"(y):);
        x = 7654321;
        y = -7654321;
        asm volatile("madd %0,%1" :: "r"(x), "r"(y):);
        asm volatile("mflo %0" : "=r"(z) ::);
        asm volatile("mfhi %0" : "=r"(w) ::);
        assert(z == -423373714);
        assert(w == -13997);
    }

    // maddu
    {
        asm volatile("mthi $0" :::);
        asm volatile("mtlo $0" :::);
        unsigned int x = 2147483649, y = 1234567, z, w;
        asm volatile("maddu %0,%1" :: "r"(x), "r"(y):);
        x = 2147483650;
        y = 7654321;
        asm volatile("maddu %0,%1" :: "r"(x), "r"(y):);
        asm volatile("mflo %0" : "=r"(z) ::);
        asm volatile("mfhi %0" : "=r"(w) ::);
        assert(z == 16543209);
        assert(w == 4444444);
    }

    // mul
    {
        int x = -1234, y = 5678, z;
        z = x * y;
        assert(z == -7006652);
    }

    // mult
    {
        int x = 1234567, y = -1234567, z, w;
        asm volatile("mult %0,%1" :: "r"(x), "r"(y):);
        asm volatile("mflo %0" : "=r"(z) ::);
        asm volatile("mfhi %0" : "=r"(w) ::);
        assert(z == 557712591);
        assert(w == -355);
    }

    // multu
    {
        unsigned int x = 2147483649, y = 2147483647, z, w;
        asm volatile("multu %0,%1" :: "r"(x), "r"(y):);
        asm volatile("mflo %0" : "=r"(z) ::);
        asm volatile("mfhi %0" : "=r"(w) ::);
        assert(z == 4294967295);
        assert(w == 1073741823);
    }

    return 0;
}
