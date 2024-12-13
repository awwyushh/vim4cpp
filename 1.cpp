Hello World 
I am Ayush Shukla!

void bubbleSort(vector<int>& arr) {
    int n = arr.size();
    for(int i = 0; i < n-1; i++) {
        for(int j = 0; j < n-i-1; j++) {
            if(arr[j] > arr[j+1]) {
                swap(arr[j], arr[j+1]);
            }
        }
    }
}

#include <bits/stdc++.h>
using namespace std;

class FastIO {
private:
    static const int BUFF_SIZE = 1 << 16;
    char input_buffer[BUFF_SIZE];
    char output_buffer[BUFF_SIZE];
    int input_pos = 0, input_len = 0;
    int output_pos = 0;
    
public:
    inline char next_char() {
        if (input_pos >= input_len) {
            input_pos = 0;
            input_len = fread(input_buffer, 1, BUFF_SIZE, stdin);
            if (input_len <= 0) return EOF;
        }
        return input_buffer[input_pos++];
    }
    
    inline int read_int() {
        int x = 0;
        char c = next_char();
        while (c <= ' ') c = next_char();
        bool neg = false;
        if (c == '-') {
            neg = true;
            c = next_char();
        }
        while (c >= '0' && c <= '9') {
            x = x * 10 + (c - '0');
            c = next_char();
        }
        return neg ? -x : x;
    }
    
    inline void write_char(char c) {
        if (output_pos >= BUFF_SIZE) {
            fwrite(output_buffer, 1, output_pos, stdout);
            output_pos = 0;
        }
        output_buffer[output_pos++] = c;
    }
    
    inline void write_int(int x) {
        if (x < 0) {
            write_char('-');
            x = -x;
        }
        char digits[20];
        int len = 0;
        do {
            digits[len++] = x % 10 + '0';
            x /= 10;
        } while (x > 0);
        while (len > 0) {
            write_char(digits[--len]);
        }
    }
    
    ~FastIO() {
        if (output_pos) {
            fwrite(output_buffer, 1, output_pos, stdout);
        }
    }
} io;

int main() {
    
    return 0;
}
